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

#[derive(Default)]
struct Tetrimino {
    kind: Option<TetriminoTypes>,
    row: usize,
    col: usize,
}

impl Tetrimino {
    fn new(tetrimino: Option<TetriminoTypes>, row: usize, col: usize) -> Self {
        Self {
            kind: tetrimino,
            row,
            col,
        }
    }
}

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;

#[derive(Default)]
pub struct Tetris {
    // Contains the entire tetris grid
    // grid[row #][col #]
    grid: [[Tetrimino; GRID_WIDTH]; GRID_HEIGHT],
    // Stores the randomized sequence of the 7 possible possible tetrimino
    bag: [TetriminoTypes; 7],
    // Stores the current tetrimino and its location in the grid
    current: Tetrimino,
    // Stores index to the held piece from the "bag"
    held: usize,
}

const O_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (1, 0), (1, 1)];
const I_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const T_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 0), (0, 1), (1, 0)];
const L_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (1, 1), (2, 1)];
const J_OFFSETS: [(i32, i32); 4] = [(0, 1), (1, 1), (2, 1), (2, 0)];
const S_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (-1, 1), (1, 0)];
const Z_OFFSETS: [(i32, i32); 4] = [(0, 0), (0, 1), (-1, 0), (1, 1)];

fn get_tetrimino_offsets(tetrimino: &TetriminoTypes) -> [(i32, i32); 4] {
    match tetrimino {
        TetriminoTypes::O => O_OFFSETS,
        TetriminoTypes::I => I_OFFSETS,
        TetriminoTypes::T => T_OFFSETS,
        TetriminoTypes::L => L_OFFSETS,
        TetriminoTypes::J => J_OFFSETS,
        TetriminoTypes::S => S_OFFSETS,
        TetriminoTypes::Z => Z_OFFSETS,
    }
}
impl Tetris {
    fn add(&mut self) {
        if let None = self.current.kind {
            return;
        }
        let kind = self.current.kind.unwrap();
        let offsets = get_tetrimino_offsets(&kind);
        for (x_off, y_off) in offsets {
            let row = (self.current.row as i32 + y_off) as usize;
            let col = (self.current.col as i32 + x_off) as usize;
            let tetrimino = Tetrimino::new(Some(kind), row, col);
            self.grid[row][col] = tetrimino;
        }
    }

    fn remove(&mut self) {
        if let None = self.current.kind {
            return;
        }
        let kind = self.current.kind.unwrap();
        let offsets = get_tetrimino_offsets(&kind);
        for (x_off, y_off) in offsets {
            let row = (self.current.row as i32 + y_off) as usize;
            let col = (self.current.col as i32 + x_off) as usize;
            let tetrimino = Tetrimino::new(None, row, col);
            self.grid[row][col] = tetrimino;
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

        let mut bag_ind = 0;
        while list.len() != 1 {
            let random_ind: usize = rng.gen_range(0..list.len());
            self.bag[bag_ind] = list.remove(random_ind);
            bag_ind += 1;
        }
        self.bag[bag_ind] = list.remove(0);
        // Set the first element to be the current 0th index element
        self.current = Tetrimino::new(Some(self.bag[0]), 0, 4);
    }

    fn soft_drop(&mut self) {
        // TODO: Find a way to to know lowest point of the current shape to use to check if tetrimino can move down
        if self.current.row < GRID_HEIGHT
            && self.grid[self.current.row + 2][self.current.col]
                .kind
                .is_none()
        {
            // Remove current tetrimino from its previous position
            self.remove();
            // Update the current tetrimino's position
            self.current.row += 1;
            // Add it at its new position
            self.add();
        }
    }
}

pub fn init(tetris: &mut Tetris) {
    tetris.generate_bag();
    tetris.add();
}

pub fn update(tetris: &mut Tetris) {
    tetris.soft_drop();
    // TODO: Find a way to delay a tetrimino's drop speed and prevent from instantly falling to the ground
}

const MINO_SIZE: f32 = ((SCREEN_HEIGHT as f32) / GRID_HEIGHT as f32) - 0.5;

pub fn render(d: &mut RaylibDrawHandle, tetris: &Tetris) {
    draw_grid(d, tetris);
    draw_next_tetrimino(d, tetris);
}

fn draw_next_tetrimino(d: &mut RaylibDrawHandle, tetris: &Tetris) {
    // let next_tetrimino = tetris.bag[tetris.current.0 + 1];
    let next_tetrimino = TetriminoTypes::Z;
    let next_tetrimino_color = get_tetrimino_color(&next_tetrimino);
    let center = Vector2::new(750.0, 80.0);
    let offsets = get_tetrimino_offsets(&next_tetrimino);
    let padding = 1.5;
    for (x_off, y_off) in offsets {
        d.draw_rectangle_v(
            Vector2::new(
                center.x + (x_off as f32 * MINO_SIZE) + (padding * x_off as f32),
                center.y + (y_off as f32 * MINO_SIZE) + (padding * y_off as f32),
                // center.y + (x_off as f32 * MINO_SIZE) + padding,
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
