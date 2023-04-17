use crate::{BACKGROUND_COLOR, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::ops::Div;

use rand::Rng;
use raylib::prelude::*;

const O_COLOR: &str = "e0d724";
const I_COLOR: &str = "8c9efa";
const T_COLOR: &str = "9b30d9";
const L_COLOR: &str = "325bad";
const J_COLOR: &str = "e88120";
const S_COLOR: &str = "32db43";
const Z_COLOR: &str = "db3232";

#[derive(Debug, Default, PartialEq, Copy, Clone)]
enum TetriminoTypes {
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

#[derive(Debug, Default, Clone)]
struct Tetrimino {
    // Positions
    row: usize,
    col: usize,
    // Type of tetrimino
    kind: Option<TetriminoTypes>,
}

impl Tetrimino {
    fn new(kind: Option<TetriminoTypes>, row: usize, col: usize) -> Self {
        Self { row, col, kind }
    }
}

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;

#[derive(Debug, Default)]
pub struct Tetris {
    // Contains the entire tetris grid
    // grid[row #][col #]
    grid: [[Tetrimino; GRID_WIDTH]; GRID_HEIGHT],
    // Stores the randomized sequence of the 7 possible possible tetrimino
    bag: [TetriminoTypes; 7],
    // Index inside the randomly generated "bag"
    bag_ind: usize,
    // Stores the current tetrimino and its location in the grid
    current: Tetrimino,
    // Stores index to the held piece from the "bag"
    held: Tetrimino,
}

const O_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];
const I_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
// const T_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 0), (0, 1), (1, 0)];
const T_OFFSETS: [(i32, i32); 4] = [(0, 0), (-1, 1), (0, 1), (1, 1)];
const L_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (1, 1), (2, 1)];
const J_OFFSETS: [(i32, i32); 4] = [(0, 1), (1, 1), (2, 1), (2, 0)];
const S_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (-1, 1), (1, 0)];
const Z_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (-1, 0), (1, 1)];

fn get_tetrimino_offsets(tetrimino: Option<TetriminoTypes>) -> [(i32, i32); 4] {
    if let Some(mino) = tetrimino {
        return match mino {
            TetriminoTypes::O => O_OFFSETS,
            TetriminoTypes::I => I_OFFSETS,
            TetriminoTypes::T => T_OFFSETS,
            TetriminoTypes::L => L_OFFSETS,
            TetriminoTypes::J => J_OFFSETS,
            TetriminoTypes::S => S_OFFSETS,
            TetriminoTypes::Z => Z_OFFSETS,
        };
    }
    [(0, 0); 4]
}
impl Tetris {
    fn add(&mut self, tetrimino: &Tetrimino) {
        if tetrimino.kind.is_none() {
            return;
        }
        let offsets = get_tetrimino_offsets(tetrimino.kind);
        for (x_off, y_off) in offsets {
            let row = tetrimino.row as i32 + y_off;
            let col = tetrimino.col as i32 + x_off;
            // Set tetrimino at current row and col to None
            if Self::is_in_bound(row, col) {
                self.grid[row as usize][col as usize] =
                    Tetrimino::new(tetrimino.kind, row as usize, col as usize);
            }
        }
    }

    fn remove(&mut self, tetrimino: &Tetrimino) {
        if tetrimino.kind.is_none() {
            return;
        }
        let offsets = get_tetrimino_offsets(tetrimino.kind);
        for (x_off, y_off) in offsets {
            let row = tetrimino.row as i32 + y_off;
            let col = tetrimino.col as i32 + x_off;
            // Set tetrimino at current row and col to None
            if Self::is_in_bound(row, col) {
                self.grid[row as usize][col as usize] =
                    Tetrimino::new(None, row as usize, col as usize);
            }
        }
    }

    fn generate_bag(&mut self) {
        let mut rng = rand::thread_rng();
        let mut list = vec![
            TetriminoTypes::O,
            TetriminoTypes::I,
            TetriminoTypes::T,
            TetriminoTypes::L,
            TetriminoTypes::J,
            TetriminoTypes::S,
            TetriminoTypes::Z,
        ];

        let mut ind = 0;
        while list.len() != 1 {
            let random_ind: usize = rng.gen_range(0..list.len());
            self.bag[ind] = list.remove(random_ind);
            ind += 1;
        }
        self.bag[ind] = list.remove(0);
        self.bag_ind = 0;
    }

    fn set_current_init(&mut self) {
        self.current.kind = Some(self.bag[0]);
    }

    fn update_current(&mut self) {
        if !self.can_move_down(&self.current) {
            let mut t = self.current.clone();
            self.add(&t);
            // Change current tetrimino to the next in line in the "bag"
            t.kind = Some(self.bag[self.bag_ind]);
            // Reset the current tetrimino's position
            t.row = 0;
            t.col = 4;
            self.current = t;
        }
    }

    fn update_bag(&mut self) {
        // TODO: Fix random piece generation
        // ISSUE: When one piece is left in the "bag": the whole thing is regenerated before the last piece is used
        if self.bag_ind + 2 >= self.bag.len() {
            self.generate_bag();
        }
        if !self.can_move_down(&self.current) {
            self.bag_ind += 1;
        }
    }

    fn is_in_bound(row: i32, col: i32) -> bool {
        (row > 0 && row < GRID_HEIGHT as i32) && (col > 0 && col < GRID_WIDTH as i32)
    }

    fn max_row_off(offset: [(i32, i32); 4], col_off: i32) -> i32 {
        let max_off = offset
            .iter()
            .filter(|x| x.0 == col_off)
            .map(|el| el.1)
            .max()
            .unwrap();
        max_off
    }

    fn can_move_down(&self, tetrimino: &Tetrimino) -> bool {
        let offsets = get_tetrimino_offsets(tetrimino.kind);
        for (x_off, _) in offsets {
            let max_y_off = Self::max_row_off(offsets, x_off);
            // The row below the current tetrimino's shape
            let next_row = (tetrimino.row as i32 + max_y_off) + 1;
            let col = tetrimino.col as i32 + x_off;
            if !Self::is_in_bound(next_row, col) {
                return false;
            }
            if self.grid[next_row as usize][col as usize].kind.is_some() {
                return false;
            }
        }
        true
    }

    fn soft_drop(&mut self) {
        // TODO: Check if a tetrimino can move down even when its rotated a certain way
        if self.can_move_down(&self.current) {
            let mut t = self.current.clone();
            // Remove current tetrimino from its previous position
            self.remove(&t);
            // Update the current tetrimino's position
            t.row += 1;
            // Add it at its new position
            self.add(&t);
            self.current = t;
        }
    }
}

pub fn init(tetris: &mut Tetris) {
    tetris.generate_bag();
    tetris.set_current_init();
}

pub fn update(tetris: &mut Tetris) {
    tetris.update_bag();
    tetris.update_current();
    tetris.soft_drop();
    // TODO: Find a way to delay a tetrimino's drop speed and prevent from instantly falling to the ground
}

const MINO_SIZE: f32 = ((SCREEN_HEIGHT as f32) / GRID_HEIGHT as f32) - 0.5;

pub fn render(d: &mut RaylibDrawHandle, tetris: &Tetris) {
    draw_grid(d, tetris);
    draw_next_tetrimino(d, tetris);
}

fn draw_next_tetrimino(d: &mut RaylibDrawHandle, tetris: &Tetris) {
    let next_tetrimino = tetris.bag[tetris.bag_ind + 1];
    // let next_tetrimino = TetriminoTypes::Z;
    let next_tetrimino_color = get_tetrimino_color(&next_tetrimino);
    let center = Vector2::new(750.0, 80.0);
    let offsets = get_tetrimino_offsets(Some(next_tetrimino));
    let padding = 1.5;
    for (x_off, y_off) in offsets {
        d.draw_rectangle_v(
            Vector2::new(
                center.x + (x_off as f32 * MINO_SIZE) + (padding * x_off as f32),
                center.y + (y_off as f32 * MINO_SIZE) + (padding * y_off as f32),
            ),
            Vector2::new(MINO_SIZE, MINO_SIZE),
            next_tetrimino_color,
        );
    }
}

fn draw_grid(d: &mut RaylibDrawHandle, tetris: &Tetris) {
    let mut padding = Vector2::new(
        SCREEN_WIDTH as f32 - (MINO_SIZE * GRID_WIDTH as f32),
        SCREEN_HEIGHT as f32 - (MINO_SIZE * GRID_HEIGHT as f32),
    );
    // Divide by 2 to center the grid horizontally and vertically
    padding = padding.div(2.0);

    for y in 0..20 {
        for x in 0..10 {
            let mut curr_color = Color::from_hex("d4d4d4").unwrap();
            if let Some(ref mino) = tetris.grid[y][x].kind {
                curr_color = get_tetrimino_color(mino);
            }
            d.draw_rectangle_v(
                Vector2::new(
                    (x as f32) * MINO_SIZE + padding.x,
                    (y as f32) * MINO_SIZE + padding.y,
                ),
                Vector2::new(MINO_SIZE, MINO_SIZE),
                curr_color,
            );
        }
    }

    // Draw horizontal lines
    for y in 0..=20 {
        d.draw_line_ex(
            Vector2::new(0.0 + padding.x, (y as f32) * MINO_SIZE + padding.y),
            Vector2::new(
                (GRID_WIDTH as f32 * MINO_SIZE) + padding.x,
                (y as f32) * MINO_SIZE + padding.y,
            ),
            2.0,
            Color::from_hex(BACKGROUND_COLOR).unwrap(),
        );
    }

    // Draw vertical lines
    for x in 0..=10 {
        d.draw_line_ex(
            Vector2::new((x as f32) * MINO_SIZE + padding.x, 0.0 + padding.y),
            Vector2::new(
                (x as f32) * MINO_SIZE + padding.x,
                GRID_HEIGHT as f32 * MINO_SIZE + padding.y,
            ),
            2.0,
            Color::from_hex(BACKGROUND_COLOR).unwrap(),
        );
    }
}

fn get_tetrimino_color(tetrimino: &TetriminoTypes) -> Color {
    let color_hex = match tetrimino {
        TetriminoTypes::O => O_COLOR,
        TetriminoTypes::I => I_COLOR,
        TetriminoTypes::T => T_COLOR,
        TetriminoTypes::L => L_COLOR,
        TetriminoTypes::J => J_COLOR,
        TetriminoTypes::S => S_COLOR,
        TetriminoTypes::Z => Z_COLOR,
    };
    Color::from_hex(color_hex).unwrap()
}
