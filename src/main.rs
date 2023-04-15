use std::ops::Div;

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 600;

const BACKGROUND_COLOR: &str = "303030";

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("tetris-rs")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // INPUT PHASE
        match handle_input() {
            _ => {}
        }

        // UPDATE PHASE
        update();

        // RENDER PHASE
        let d = rl.begin_drawing(&thread);
        render(d);
    }
}

enum Actions {
    Pause,
}

fn handle_input() -> Option<Actions> {
    None
}

fn update() {}

fn render(mut d: RaylibDrawHandle) {
    d.clear_background(Color::from_hex(BACKGROUND_COLOR).unwrap());
    draw_grid(&mut d);
}

fn draw_grid(d: &mut RaylibDrawHandle) {
    let mino_size = ((SCREEN_HEIGHT as f32) / 20.0) - 0.5;
    let mut padding = Vector2::new(
        SCREEN_WIDTH as f32 - (mino_size * 10.0),
        SCREEN_HEIGHT as f32 - (mino_size * 20.0),
    );
    // Divide by 2 to center the grid horizontally and vertically
    padding = padding.div(2.0);

    for y in 0..20 {
        for x in 0..10 {
            d.draw_rectangle_v(
                Vector2::new(
                    (x as f32) * mino_size + padding.x,
                    (y as f32) * mino_size + padding.y,
                ),
                Vector2::new(mino_size, mino_size),
                Color::from_hex("dddddd").unwrap(),
            );
        }
    }

    for y in 0..=20 {
        d.draw_line_ex(
            Vector2::new(0.0 + padding.x, (y as f32) * mino_size + padding.y),
            Vector2::new(
                (10.0 * mino_size) + padding.x,
                (y as f32) * mino_size + padding.y,
            ),
            2.0,
            Color::GRAY,
        );
    }

    for x in 0..=10 {
        d.draw_line_ex(
            Vector2::new((x as f32) * mino_size + padding.x, 0.0 + padding.y),
            Vector2::new((x as f32) * mino_size + padding.x, 20.0 * mino_size + padding.y),
            2.0,
            Color::GRAY,
        );
    }
}
