use std::{fmt::Display, ops::Not};

use rand::Rng;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    Blue,
    Red,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Piece::Blue => "Blue",
                Piece::Red => "Red",
            }
        )
    }
}

impl Not for Piece {
    type Output = Piece;

    fn not(self) -> Self::Output {
        match self {
            Piece::Blue => Piece::Red,
            Piece::Red => Piece::Blue,
        }
    }
}

impl Piece {
    pub fn rand() -> Self {
        if rand::thread_rng().gen_bool(0.5) {
            Piece::Blue
        } else {
            Piece::Red
        }
    }
}

impl Into<char> for Piece {
    fn into(self) -> char {
        match self {
            Piece::Blue => 'B',
            Piece::Red => 'R',
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::piece::Piece;

    #[test]
    fn test_rand() {
        let p = Piece::rand();
        assert!(Piece::rand() == Piece::Blue || p == Piece::Red)
    }

    #[test]
    fn test_display() {
        assert_eq!(Piece::Red.to_string(), "Red");
        assert_eq!(Piece::Blue.to_string(), "Blue");
    }

    #[test]
    fn test_not_piece() {
        assert_eq!(!Piece::Red, Piece::Blue);
        assert_eq!(!Piece::Blue, Piece::Red);
    }

    #[test]
    fn test_into_char() {
        let red_c: char = Piece::Red.into();
        let blue_c: char = Piece::Blue.into();
        assert_eq!(red_c, 'R');
        assert_eq!(blue_c, 'B');
    }
}
