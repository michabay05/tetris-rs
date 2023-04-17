use raylib::prelude::*;

mod tetris;
use tetris::Tetris;

const SCREEN_WIDTH: i32 = 1000;
const SCREEN_HEIGHT: i32 = 600;

const BACKGROUND_COLOR: &str = "303030";

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("tetris-rs")
        // .msaa_4x()
        .build();

    rl.set_target_fps(10);

    let mut tetris = Tetris::default();
    init_game(&mut tetris);

    while !rl.window_should_close() {
        // INPUT PHASE
        match handle_input() {
            _ => {}
        }

        // UPDATE PHASE
        update(&mut tetris);

        // RENDER PHASE
        let d = rl.begin_drawing(&thread);
        render(d, &tetris);
    }
}

fn init_game(tetris: &mut Tetris) {
    tetris::init(tetris);
}

enum Actions {
    Pause,
}

fn handle_input() -> Option<Actions> {
    None
}

fn update(tetris: &mut Tetris) {
    tetris::update(tetris);
}

fn render(mut d: RaylibDrawHandle, tetris: &Tetris) {
    d.clear_background(Color::from_hex(BACKGROUND_COLOR).unwrap());
    tetris::render(&mut d, tetris);
}
