use std::println;

use macroquad::prelude::*;
use macroquad::rand::{RandomRange, srand};
use std::time::{SystemTime, UNIX_EPOCH};

#[macroquad::main("BasicShapes")]
async fn main() {
    // Seed
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    srand(seed);

    // Rectangle
    let mut x_rectangle = screen_width() / 2.0 - 60.0;
    let y_rectangle = screen_height() / 2. + 225.;

    let mut score = 0;
    // Ball
    const BALL_RADIUS:f32 = 8.;
    let x_circle = RandomRange::gen_range(10., screen_width());
    let y_circle = RandomRange::gen_range(10., screen_height());
    let (dx, dy) = random_ball_direction(3.0, seed); // 3.0 — это "скорость" (общая длина вектора движения)
    let mut ball = Ball::new(x_circle, y_circle, dx, dy);

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::A) {
            x_rectangle -= 4.;
        }

        if is_key_down(KeyCode::D) {
            x_rectangle += 4.;
        }

        if x_rectangle <= 0. {
            x_rectangle = 0.
        }

        if x_rectangle >= 700. {
            x_rectangle = 700.
        }

        ball.x += ball.dx;
        ball.y += ball.dy;

        let rect_left = x_rectangle;
        let rect_right = x_rectangle + 100.0;
        let rect_top = y_rectangle;
        let rect_bottom = y_rectangle + 10.0;

        let closest_x = ball.x.clamp(rect_left, rect_right);
        let closest_y = ball.y.clamp(rect_top, rect_bottom);

        let dist_x = ball.x - closest_x;
        let dist_y = ball.y - closest_y;
        let distance_squared = dist_x * dist_x + dist_y * dist_y;

        if distance_squared <= BALL_RADIUS * BALL_RADIUS {
            let overlap_x = BALL_RADIUS - dist_x.abs();
            let overlap_y = BALL_RADIUS - dist_y.abs();

            if overlap_x < overlap_y {
                ball.dx = -ball.dx;
            } else {
                ball.dy = -ball.dy;
            }
        }

        if ball.x - BALL_RADIUS <= 0.0 || ball.x + BALL_RADIUS >= screen_width() {
            ball.dx = -ball.dx;
        }

        // аналогично для верхней/нижней стены (если нужно)
        if ball.y - BALL_RADIUS <= 0.0 || ball.y + BALL_RADIUS >= screen_height() {
            ball.dy = -ball.dy;
        }

        draw_text(score.to_string(), 20., 30., 35., WHITE);
        draw_circle(ball.x, ball.y, BALL_RADIUS, YELLOW);
        draw_rectangle(x_rectangle, y_rectangle, 100.0, 10.0, WHITE);

        next_frame().await
    }
}

struct Ball {
    x: f32,
    y: f32,
    dx: f32, // сдвиг по X за кадр
    dy: f32, // сдвиг по Y за кадр
}

impl Ball {
    fn new(x: f32, y: f32, dx: f32, dy: f32) -> Self {
        Self {x, y, dx, dy}
    }
}

fn random_ball_direction(speed: f32, seed: u64) -> (f32, f32) {
    srand(seed);

    // случайный угол в диапазоне 30°-60°, чтобы избежать чисто горизонтального/вертикального движения
    let angle_deg: f32 = RandomRange::gen_range(30.0, 60.0);
    let angle_rad = angle_deg.to_radians();

    let mut dx = speed * angle_rad.cos();
    let mut dy = speed * angle_rad.sin();

    // случайно выбираем направление по каждой оси
    if RandomRange::gen_range(0, 2) == 0 {
        dx = -dx;
    }
    if RandomRange::gen_range(0, 2) == 0 {
        dy = -dy;
    }

    (dx, dy)
}