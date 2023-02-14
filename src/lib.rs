pub mod board;
pub mod coordinates;
pub mod game;
pub mod piece;
pub mod position;
pub mod walker;
pub struct Wrap<T>(pub T);

pub type Dir = Direction;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}
