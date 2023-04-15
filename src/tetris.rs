use crate::{BACKGROUND_COLOR, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::ops::Div;

use raylib::prelude::*;

const O_COLOR: &str = "e0d724";
const I_COLOR: &str = "8c9efa";
const T_COLOR: &str = "9b30d9";
const L_COLOR: &str = "325bad";
const J_COLOR: &str = "e88120";
const S_COLOR: &str = "32db43";
const Z_COLOR: &str = "db3232";

#[derive(Default, PartialEq, Copy, Clone)]
enum Tetrimino {
    // TODO: Find a way to fix this '#[default]' because the O-tetrimino is obviously not the default
    #[default]
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;

#[derive(Default)]
pub struct Tetris {
    // grid[row #][col #]
    grid: [[Option<Tetrimino>; GRID_WIDTH]; GRID_HEIGHT],
    current: Tetrimino,
    held: Tetrimino,
}

const O_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];
const I_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const T_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 0), (0, 1), (1, 0)];
const L_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (1, 1), (2, 1)];
const J_OFFSETS: [(i32, i32); 4] = [(0, 1), (1, 1), (2, 1), (2, 0)];
const S_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (-1, 1), (1, 0)];
const Z_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (-1, 0), (1, 1)];

impl Tetris {
    fn add(&mut self, tetrimino: Tetrimino) -> bool {
        let t = Tetrimino::Z;
        let offsets = match t {
            Tetrimino::O => O_OFFSETS,
            Tetrimino::I => I_OFFSETS,
            Tetrimino::T => T_OFFSETS,
            Tetrimino::L => L_OFFSETS,
            Tetrimino::J => J_OFFSETS,
            Tetrimino::S => S_OFFSETS,
            Tetrimino::Z => Z_OFFSETS,
        };
        let center = (0, 4);
        for (x_off, y_off) in offsets {
            self.grid[(center.0 + y_off) as usize][(center.1 + x_off) as usize] = Some(t);
        }
        return true;
    }

    fn soft_drop(&mut self) {
        todo!("Implement soft dropping for current tetrimino");
    }

    fn preview_drop(&self) {
        todo!(
            "Implement a ghost piece that shows the preview of where the current piece will land"
        );
    }
}

pub fn update(tetris: &mut Tetris) {
    tetris.add(Tetrimino::L);
}

pub fn render(d: &mut RaylibDrawHandle, tetris: &Tetris) {
    draw_grid(d, tetris);
}

fn draw_grid(d: &mut RaylibDrawHandle, tetris: &Tetris) {
    let mino_size = ((SCREEN_HEIGHT as f32) / GRID_HEIGHT as f32) - 0.5;
    let mut padding = Vector2::new(
        SCREEN_WIDTH as f32 - (mino_size * GRID_WIDTH as f32),
        SCREEN_HEIGHT as f32 - (mino_size * GRID_HEIGHT as f32),
    );
    // Divide by 2 to center the grid horizontally and vertically
    padding = padding.div(2.0);

    for y in 0..20 {
        for x in 0..10 {
            let mut curr_color: &str = "d4d4d4";
            if let Some(ref cell) = tetris.grid[y][x] {
                curr_color = match cell {
                    Tetrimino::O => O_COLOR,
                    Tetrimino::I => I_COLOR,
                    Tetrimino::T => T_COLOR,
                    Tetrimino::L => L_COLOR,
                    Tetrimino::J => J_COLOR,
                    Tetrimino::S => S_COLOR,
                    Tetrimino::Z => Z_COLOR,
                };
            }
            d.draw_rectangle_v(
                Vector2::new(
                    (x as f32) * mino_size + padding.x,
                    (y as f32) * mino_size + padding.y,
                ),
                Vector2::new(mino_size, mino_size),
                Color::from_hex(curr_color).unwrap(),
            );
        }
    }

    // Draw horizontal lines
    for y in 0..=20 {
        d.draw_line_ex(
            Vector2::new(0.0 + padding.x, (y as f32) * mino_size + padding.y),
            Vector2::new(
                (GRID_WIDTH as f32 * mino_size) + padding.x,
                (y as f32) * mino_size + padding.y,
            ),
            2.0,
            Color::from_hex(BACKGROUND_COLOR).unwrap(),
        );
    }

    // Draw vertical lines
    for x in 0..=10 {
        d.draw_line_ex(
            Vector2::new((x as f32) * mino_size + padding.x, 0.0 + padding.y),
            Vector2::new(
                (x as f32) * mino_size + padding.x,
                GRID_HEIGHT as f32 * mino_size + padding.y,
            ),
            2.0,
            Color::from_hex(BACKGROUND_COLOR).unwrap(),
        );
    }
}
