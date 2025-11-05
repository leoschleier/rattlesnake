use macroquad::{color, input, math, shapes, text, time, window};
use rattlesnake::PlayerEvent;

pub struct BrowserUI {
    field_x: u16,
    field_y: u16,
    field_width: u16,
    field_height: u16,
    thickness: u16,
    base_thickness: u16,
    score: Score,
    arrow_keys: ArrowKeys,
    touch_toggle: ToggleButton,
}

pub struct ArrowKeys {
    left: RectButton,
    right: RectButton,
    up: RectButton,
    down: RectButton,
    is_visible: bool,
}

pub struct ToggleButton {
    button: RectButton,
    is_on: bool,
}

struct RectButton {
    rect: math::Rect,
    text: Text,
    event: PlayerEvent,
}

struct Text {
    content: &'static str,
    font_size: u16,
}

struct Score {
    text: Text,
    value: u8,
}

impl BrowserUI {
    pub fn new(width: u16, height: u16, thickness: u16) -> Self {
        let score = Score {
            text: Text {
                content: "Score: ",
                font_size: 40,
            },
            value: 0,
        };

        BrowserUI {
            field_x: 0,
            field_y: 0,
            field_width: width,
            field_height: height,
            thickness,
            base_thickness: thickness,
            score,
            arrow_keys: create_arrow_keys(),
            touch_toggle: create_touch_toggle(100.0),
        }
    }

    pub fn update_positions(&mut self) {
        let screen_width = window::screen_width();
        let screen_height = window::screen_height();

        let arrow_key_size = 100.0;

        let base_width = ((self.field_width + 2) * self.base_thickness) as f32;
        let base_height = ((self.field_height + 2) * self.base_thickness)
            as f32
            + arrow_key_size * 4.0;

        let scale =
            (screen_width / base_width).min(screen_height / base_height);

        self.thickness = (self.base_thickness as f32 * scale) as u16;

        self.field_x = ((screen_width
            - ((self.field_width + 2) * self.thickness) as f32)
            / 2.0) as u16;
        self.field_y = arrow_key_size as u16 + 2 * self.thickness;

        self.update_arrow_keys_position(arrow_key_size);
        self.update_touch_toggle_position();
    }

    pub fn update_arrow_keys_position(&mut self, size: f32) {
        // Add 2 to width to account for field border
        let x = (self.field_x + ((self.field_width + 2) * self.thickness) / 2)
            as f32;
        let y = (self.field_y + self.field_height * self.thickness) as f32
            + 2.0 * size;
        let space = size / 4.0;
        let first_row = y + space;
        let second_row = first_row + size + space;
        let center_column = x - size / 2.0;
        let left_column = center_column - size - space;
        let right_column = center_column + size + space;

        self.arrow_keys.up.rect.x = center_column;
        self.arrow_keys.up.rect.y = first_row;
        self.arrow_keys.down.rect.x = center_column;
        self.arrow_keys.down.rect.y = second_row;
        self.arrow_keys.left.rect.x = left_column;
        self.arrow_keys.left.rect.y = second_row;
        self.arrow_keys.right.rect.x = right_column;
        self.arrow_keys.right.rect.y = second_row;

        for button in [
            &mut self.arrow_keys.up,
            &mut self.arrow_keys.down,
            &mut self.arrow_keys.left,
            &mut self.arrow_keys.right,
        ] {
            button.rect.w = size;
            button.rect.h = size;
        }
    }

    pub fn update_touch_toggle_position(&mut self) {
        self.touch_toggle.button.rect.x =
            (self.field_x + self.field_width * self.thickness) as f32
                + 2.0 * self.thickness as f32
                - self.touch_toggle.button.rect.w;

        self.touch_toggle.button.rect.y =
            self.field_y.saturating_sub(self.thickness) as f32
                - self.touch_toggle.button.rect.h;
    }

    pub fn render(
        &mut self,
        snake: &[(u16, u16)],
        food: &Vec<(u16, u16)>,
        score: u8,
    ) {
        self.update_positions();

        self.draw_field(
            self.field_x,
            self.field_y,
            self.field_width,
            self.field_height,
            self.thickness,
        );

        self.draw_touch_toggle();
        self.draw_arrow_keys();

        for f in food {
            self.draw_food(&(self.field_x, self.field_y), f, self.thickness);
        }

        for s in snake {
            self.draw_snake(&(self.field_x, self.field_y), s, self.thickness);
        }

        self.score.value = score;
        self.draw_score(
            self.field_x,
            self.field_y.saturating_sub(self.thickness),
        );
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

    fn draw_score(&self, x: u16, y: u16) {
        let score_text =
            format!("{}{}", self.score.text.content, self.score.value);
        text::draw_text(
            &score_text,
            x as f32,
            y as f32,
            self.score.text.font_size as f32,
            macroquad::color::WHITE,
        );
    }

    fn draw_arrow_keys(&mut self) {
        if !self.arrow_keys.is_visible {
            return;
        }
        let arrow_keys = [
            &self.arrow_keys.up,
            &self.arrow_keys.down,
            &self.arrow_keys.left,
            &self.arrow_keys.right,
        ];

        for button in &arrow_keys {
            let rect = &button.rect;
            shapes::draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                color::WHITE,
            );
            draw_text_centered(
                rect.x,
                rect.y,
                button,
                button.text.font_size,
                color::BLACK,
            );
        }
    }

    fn draw_touch_toggle(&self) {
        let button = &self.touch_toggle.button;
        let mut text_color = color::WHITE;
        if self.touch_toggle.is_on {
            shapes::draw_rectangle(
                button.rect.x,
                button.rect.y,
                button.rect.w,
                button.rect.h,
                color::WHITE,
            );
            text_color = color::BLACK;
        } else {
            shapes::draw_rectangle_lines(
                button.rect.x,
                button.rect.y,
                button.rect.w,
                button.rect.h,
                4.0,
                color::WHITE,
            );
        }

        draw_text_centered(
            button.rect.x,
            button.rect.y,
            button,
            button.text.font_size,
            text_color,
        );
    }

    pub fn poll(&mut self, millis: u64, prev: &PlayerEvent) -> PlayerEvent {
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
                if self.arrow_keys.is_visible {
                    let arrow_keys = [
                        &self.arrow_keys.up,
                        &self.arrow_keys.down,
                        &self.arrow_keys.left,
                        &self.arrow_keys.right,
                    ];
                    for button in arrow_keys {
                        if button.rect.contains(touch) {
                            event = button.event;
                            break;
                        }
                    }
                }
                if self.touch_toggle.button.rect.contains(touch) {
                    self.touch_toggle.is_on = !self.touch_toggle.is_on;
                    self.arrow_keys.is_visible = self.touch_toggle.is_on;
                    self.draw_touch_toggle();
                    self.draw_arrow_keys();
                    event = PlayerEvent::ToggleArrowKeys;
                    break;
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

fn create_arrow_keys() -> ArrowKeys {
    // Arrow Keys
    let arrow_key_size = 80.0;
    let left_arrow_text = Text {
        content: "<",
        font_size: 60,
    };
    let left = RectButton {
        rect: math::Rect::new(0.0, 0.0, arrow_key_size, arrow_key_size),
        text: left_arrow_text,
        event: PlayerEvent::Left,
    };
    let right_arrow_text = Text {
        content: ">",
        font_size: 60,
    };
    let right = RectButton {
        rect: math::Rect::new(0.0, 0.0, arrow_key_size, arrow_key_size),
        text: right_arrow_text,
        event: PlayerEvent::Right,
    };
    let up_arrow_text = Text {
        content: "^",
        font_size: 60,
    };
    let up = RectButton {
        rect: math::Rect::new(0.0, 0.0, arrow_key_size, arrow_key_size),
        text: up_arrow_text,
        event: PlayerEvent::Up,
    };
    let down_arrow_text = Text {
        content: "v",
        font_size: 60,
    };
    let down = RectButton {
        rect: math::Rect::new(0.0, 0.0, arrow_key_size, arrow_key_size),
        text: down_arrow_text,
        event: PlayerEvent::Down,
    };

    ArrowKeys {
        left,
        right,
        up,
        down,
        is_visible: false,
    }
}

fn create_touch_toggle(size: f32) -> ToggleButton {
    // Touch Toggle Button
    let touch_text = Text {
        content: "Touch",
        font_size: 40,
    };
    let button = RectButton {
        rect: math::Rect::new(0.0, 0.0, size, size),
        text: touch_text,
        event: PlayerEvent::ToggleArrowKeys,
    };
    ToggleButton {
        button,
        is_on: false,
    }
}

fn draw_text_centered(
    x: f32,
    y: f32,
    button: &RectButton,
    font_size: u16,
    color: color::Color,
) {
    let text_dim =
        text::measure_text(button.text.content, None, font_size, 1.0);
    let text_x = x + (button.rect.w - text_dim.width) / 2.0;
    let text_y = y + (button.rect.h + text_dim.height) / 2.0;
    text::draw_text(
        button.text.content,
        text_x,
        text_y,
        font_size as f32,
        color,
    );
}

pub fn now_millis() -> u64 {
    (time::get_time() * 1000.0) as u64
}
