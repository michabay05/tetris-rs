# tetris-rs

## Quickstart

Make sure `cargo` is installed beforehand

```
git clone https://github.com/michabay05/tetris-rs
cd tetris-rs
cargo run
```

## ROADMAP

- [x] Grid
  - [x] Creating grid
  - [x] Display grid
- [ ] Tetrimonioes
  - [x] Spawn tetrimino
  - [x] Draw(Render) tetrimino
  - [ ] Soft Drop
    - [x] Check if piece can move down first
    - [ ] Drop with correct drop speed
  - [ ] Preview landing spot
- [ ] Random "bag" - tetriminoes generation
  - [x] Generate a list of tetriminoes in a "virtual bag"
  - [x] Show next tetrimino
  - [ ] Show held tetrimino
  - [x] Update bag when the current tetrimino hits the floor
- [ ] Controls
  - [ ] Move tetrimino
    - [ ] Move left and right
    - [ ] Hard Drop
    - [ ] Soft Drop (Additional on down key press) 
  - [ ] Rotate tetrimino
  - [ ] Hold tetrimino

## Sources

- [2009 Tetris Design Guidelines](https://github.com/frankkopp/Tetris/blob/master/2009%20Tetris%20Design%20Guideline.pdf)
