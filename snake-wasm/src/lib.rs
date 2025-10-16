use macroquad::{input, shapes};
use rattlesnake::PlayerEvent;

pub struct BrowserUI {
    width: u16,
    height: u16,
    thickness: u16,
}

impl BrowserUI {
    pub fn new(width: u16, height: u16, thickness: u16) -> Self {
        BrowserUI {
            width,
            height,
            thickness,
        }
    }

    pub fn render(&mut self, snake: &[(u16, u16)], food: &Vec<(u16, u16)>) {
        let anchor_x = ((macroquad::window::screen_width()
            - (self.width * self.thickness + 2 * self.thickness) as f32)
            / 2.0) as u16;
        let anchor_y = ((macroquad::window::screen_height()
            - (self.height * self.thickness + 2 * self.thickness) as f32)
            / 2.0) as u16;

        self.draw_field(
            anchor_x,
            anchor_y,
            self.width,
            self.height,
            self.thickness,
        );

        for f in food {
            self.draw_food(&(anchor_x, anchor_y), f, self.thickness);
        }

        for s in snake {
            self.draw_snake(&(anchor_x, anchor_y), s, self.thickness);
        }
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
            macroquad::color::GREEN,
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

    pub fn poll(&self, millis: u64, prev: &PlayerEvent) -> PlayerEvent {
        let start = std::time::Instant::now();
        let mut event = PlayerEvent::Idle;
        loop {
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
            let is_timeout_reached =
                start.elapsed().as_millis() >= millis as u128;
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
