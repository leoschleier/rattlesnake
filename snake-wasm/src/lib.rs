use macroquad::{color, input, math, shapes, text, time, window};
use rattlesnake::PlayerEvent;

pub struct BrowserUI {
    width: u16,
    height: u16,
    thickness: u16,
    buttons: Vec<Button>,
}

struct Button {
    rect: math::Rect,
    label: &'static str,
    event: PlayerEvent,
}

impl BrowserUI {
    pub fn new(width: u16, height: u16, thickness: u16) -> Self {
        BrowserUI {
            width,
            height,
            thickness,
            buttons: vec![],
        }
    }

    pub fn render(
        &mut self,
        snake: &[(u16, u16)],
        food: &Vec<(u16, u16)>,
        score: u8,
    ) {
        let screen_width = window::screen_width();
        let screen_height = window::screen_height();

        let anchor_x = ((screen_width
            - (self.width * self.thickness + 2 * self.thickness) as f32)
            / 2.0) as u16;
        let anchor_y = ((screen_height
            - (self.height * self.thickness + 2 * self.thickness) as f32)
            / 2.0) as u16;

        self.draw_field(
            anchor_x,
            anchor_y,
            self.width,
            self.height,
            self.thickness,
        );

        self.draw_touch_controls(
            screen_width / 2.0,
            (anchor_y + self.height * self.thickness) as f32 + 50.0,
            80.0,
        );

        for f in food {
            self.draw_food(&(anchor_x, anchor_y), f, self.thickness);
        }

        for s in snake {
            self.draw_snake(&(anchor_x, anchor_y), s, self.thickness);
        }

        self.draw_score(anchor_x, anchor_y - self.thickness, score);
    }

    fn draw_field(
        &self,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        thickness: u16,
    ) {
        shapes::draw_rectangle_lines(
            x as f32,
            y as f32,
            (width * thickness + 2 * thickness) as f32,
            (height * thickness + 2 * thickness) as f32,
            thickness as f32,
            macroquad::color::WHITE,
        );
    }

    fn draw_snake(
        &self,
        anchor: &(u16, u16),
        pos: &(u16, u16),
        thickness: u16,
    ) {
        shapes::draw_rectangle(
            (anchor.0 + pos.0 * thickness) as f32,
            (anchor.1 + pos.1 * thickness) as f32,
            thickness as f32,
            thickness as f32,
            macroquad::color::GREEN,
        );
    }

    fn draw_food(&self, anchor: &(u16, u16), pos: &(u16, u16), thickness: u16) {
        shapes::draw_rectangle(
            (anchor.0 + pos.0 * thickness) as f32,
            (anchor.1 + pos.1 * thickness) as f32,
            thickness as f32,
            thickness as f32,
            macroquad::color::WHITE,
        );
    }

    fn draw_score(&self, x: u16, y: u16, score: u8) {
        let score_text = format!("Score: {}", score);
        text::draw_text(
            &score_text,
            x as f32,
            y as f32,
            30.0,
            macroquad::color::WHITE,
        );
    }

    fn draw_touch_controls(&mut self, x: f32, y: f32, size: f32) {
        let space = size / 4.0;
        let first_row = y + space;
        let second_row = first_row + size + space;
        let left_column = x - 1.5 * size - space;
        let center_column = x - size / 2.0;
        let right_column = x + size / 2.0 + space;

        // Button positions
        let left = Button {
            rect: math::Rect::new(left_column, second_row, size, size),
            label: "<",
            event: PlayerEvent::Left,
        };
        let right = Button {
            rect: math::Rect::new(right_column, second_row, size, size),
            label: ">",
            event: PlayerEvent::Right,
        };
        let up = Button {
            rect: math::Rect::new(center_column, first_row, size, size),
            label: "^",
            event: PlayerEvent::Up,
        };
        let down = Button {
            rect: math::Rect::new(center_column, second_row, size, size),
            label: "v",
            event: PlayerEvent::Down,
        };

        self.buttons = vec![left, right, up, down];

        // Draw Buttons
        let font_size = 60;
        for button in &self.buttons {
            shapes::draw_rectangle(
                button.rect.x,
                button.rect.y,
                button.rect.w,
                button.rect.h,
                color::WHITE,
            );

            let text_dims =
                text::measure_text(button.label, None, font_size, 1.0);
            let text_x =
                button.rect.x + (button.rect.w - text_dims.width) / 2.0;
            let text_y =
                button.rect.y + (button.rect.h + text_dims.height) / 2.0;
            text::draw_text(
                button.label,
                text_x,
                text_y,
                font_size as f32,
                color::BLACK,
            );
        }
    }

    pub fn poll(&self, millis: u64, prev: &PlayerEvent) -> PlayerEvent {
        let start = now_millis();
        let mut event = PlayerEvent::Idle;
        loop {
            // Handle keyboard input
            if self.is_pressed(macroquad::prelude::KeyCode::Up) {
                event = PlayerEvent::Up;
            }
            if self.is_pressed(macroquad::prelude::KeyCode::Down) {
                event = PlayerEvent::Down;
            }
            if self.is_pressed(macroquad::prelude::KeyCode::Left) {
                event = PlayerEvent::Left;
            }
            if self.is_pressed(macroquad::prelude::KeyCode::Right) {
                event = PlayerEvent::Right;
            }
            if self.is_pressed(macroquad::prelude::KeyCode::Escape) {
                event = PlayerEvent::Quit;
            }
            // Handle mouse/touch input
            if input::is_mouse_button_pressed(input::MouseButton::Left) {
                let pos = input::mouse_position();
                let touch = math::Vec2::new(pos.0, pos.1);
                for button in &self.buttons {
                    if button.rect.contains(touch) {
                        event = button.event;
                    }
                }
            }
            let is_timeout_reached = now_millis() - start >= millis;
            let is_new_event = event != PlayerEvent::Idle && event != *prev;
            if is_timeout_reached || is_new_event {
                break;
            }
        }
        event
    }

    fn is_pressed(&self, key: macroquad::prelude::KeyCode) -> bool {
        input::is_key_pressed(key)
    }

    pub async fn flush(&self) {
        macroquad::window::next_frame().await;
    }
}

pub fn now_millis() -> u64 {
    (time::get_time() * 1000.0) as u64
}
