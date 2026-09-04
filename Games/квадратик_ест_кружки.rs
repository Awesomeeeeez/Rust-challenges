use std::println;

use macroquad::prelude::*;
use macroquad::rand::{RandomRange, srand};
use std::time::{SystemTime, UNIX_EPOCH};

#[macroquad::main("BasicShapes")]
async fn main() {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    srand(seed);

    let mut x_rectangle = screen_width() / 2.0 - 60.0;
    let mut y_rectangle = 100.;
    let mut x_circle = RandomRange::gen_range(10., screen_width());
    let mut y_circle = RandomRange::gen_range(10., screen_height());

    let mut score = 0;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::W) {
            y_rectangle -= 1.;
        }

        if is_key_down(KeyCode::S) {
            y_rectangle += 1.;
        }

        if is_key_down(KeyCode::A) {
            x_rectangle -= 1.;
        }

        if is_key_down(KeyCode::D) {
            x_rectangle += 1.;
        }

        let distance = ((x_rectangle - x_circle).powi(2) + (y_rectangle - y_circle).powi(2)).sqrt();

        if distance < 20.0 { // подбери число под размеры своих фигур
            score += 1;
            x_circle = RandomRange::gen_range(10., screen_width());
            y_circle = RandomRange::gen_range(10., screen_height());
        }

        if x_rectangle == screen_width() {
            x_rectangle = 10.;
        }

        if x_rectangle == 0. {
            x_rectangle = screen_width() - 10.;
        }

        if y_rectangle == screen_height() {
            y_rectangle = 10.;
        }

        if y_rectangle == 0. {
            y_rectangle = screen_height() - 10.;
        }

        draw_text(score.to_string(), 20., 30., 35., WHITE);
        draw_circle(x_circle, y_circle, 8.0, YELLOW);
        draw_rectangle(x_rectangle, y_rectangle, 25.0, 25.0, RED);

        next_frame().await
    }
}