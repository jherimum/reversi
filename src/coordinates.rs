use regex::Regex;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum CoordinatesError {
    #[error("Invalid coordinates format: '{0}'")]
    ParseError(String),
}

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

pub struct CoordinatesIterator<'c>(&'c Coordinates, Direction);

impl<'c> Iterator for CoordinatesIterator<'c> {
    type Item = Coordinates;

    fn next(&mut self) -> Option<Self::Item> {
        match self.1 {
            Direction::Up => match (self.0.row, self.0.col) {
                (0, _) => None,
                (r, c) => Some(Coordinates { row: r - 1, col: c }),
            },
            Direction::UpRight => match (self.0.row, self.0.col) {
                (0, _) => None,
                (r, c) => Some(Coordinates::new(r - 1, c + 1)),
            },

            Direction::Right => Some(Coordinates::new(self.0.row, self.0.col + 1)),

            Direction::DownRight => Some(Coordinates::new(self.0.row + 1, self.0.col + 1)),

            Direction::Down => Some(Coordinates::new(self.0.row + 1, self.0.col)),

            Direction::DownLeft => match (self.0.row, self.0.col) {
                (_, 0) => None,
                (r, c) => Some(Coordinates::new(r + 1, c - 1)),
            },
            Direction::Left => match (self.0.row, self.0.col) {
                (_, 0) => None,
                (r, c) => Some(Coordinates::new(r, c - 1)),
            },
            Direction::UpLeft => match (self.0.row, self.0.col) {
                (0, _) => None,
                (_, 0) => None,
                (r, c) => Some(Coordinates::new(r - 1, c - 1)),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinates {
    pub row: usize,
    pub col: usize,
}

impl Coordinates {
    pub fn new(row: usize, col: usize) -> Self {
        Coordinates { row, col }
    }

    pub fn iterator(&self, direction: Direction) -> CoordinatesIterator {
        CoordinatesIterator(self, direction)
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", RowIdentifier(self.row), self.col + 1)
    }
}

impl Into<String> for Coordinates {
    fn into(self) -> String {
        self.to_string()
    }
}

impl FromStr for Coordinates {
    type Err = CoordinatesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"\A(?P<row>[A-Z]+):(?P<col>\d+)\z").expect("Invalid regex");

        let captures = regex
            .captures(s)
            .ok_or(CoordinatesError::ParseError(s.to_string()))?;

        let col: usize = captures
            .name("col")
            .expect("a capture with name col was expected")
            .as_str()
            .parse()
            .expect("Failed to parse col. a number was expected");

        let row = RowIdentifier::from_str(
            captures
                .name("row")
                .expect("a capture with name row was expected")
                .as_str(),
        )?
        .0;

        Ok(Coordinates {
            row: row,
            col: col - 1,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RowIdentifier(usize);

impl Display for RowIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut id = String::new();
        let mut temp_row = self.0 + 1;

        while temp_row > 0 {
            let letter = (temp_row - 1) % 26;
            id.push((letter + 65) as u8 as char);
            temp_row = (temp_row - 1) / 26;
        }

        write!(f, "{}", id.chars().rev().collect::<String>())
    }
}

impl FromStr for RowIdentifier {
    type Err = CoordinatesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valid_regex = Regex::new(r"\A([A-Z]|[a-z])+\z").expect("regex error");
        if !valid_regex.is_match(s) {
            return Err(CoordinatesError::ParseError(s.to_string()));
        }

        let mut res: usize = 0;
        for c in s.chars() {
            res = res * 26;
            res = res + ((c.to_ascii_uppercase() as u8) - ('A' as u8) + 1) as usize;
        }

        Ok(RowIdentifier(res - 1))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::coordinates::{CoordinatesError, RowIdentifier};

    use super::Coordinates;

    #[test]
    fn test_row_identifier_display() {
        assert_eq!(RowIdentifier(0).to_string(), String::from("A"));
        assert_eq!(RowIdentifier(1).to_string(), String::from("B"));
        assert_eq!(RowIdentifier(25).to_string(), String::from("Z"));
        assert_eq!(RowIdentifier(26).to_string(), String::from("AA"));
        assert_eq!(RowIdentifier(27).to_string(), String::from("AB"));
        assert_eq!(RowIdentifier(28).to_string(), String::from("AC"));
        assert_eq!(RowIdentifier(30).to_string(), String::from("AE"));
        assert_eq!(RowIdentifier(40).to_string(), String::from("AO"));
        assert_eq!(RowIdentifier(45).to_string(), String::from("AT"));
        assert_eq!(RowIdentifier(46).to_string(), String::from("AU"));
        assert_eq!(RowIdentifier(47).to_string(), String::from("AV"));
        assert_eq!(RowIdentifier(48).to_string(), String::from("AW"));
        assert_eq!(RowIdentifier(49).to_string(), String::from("AX"));
        assert_eq!(RowIdentifier(50).to_string(), String::from("AY"));
        assert_eq!(RowIdentifier(51).to_string(), String::from("AZ"));
        assert_eq!(RowIdentifier(52).to_string(), String::from("BA"));
    }

    #[test]
    fn test_row_identifier_from_str() {
        assert_eq!(RowIdentifier::from_str("a").unwrap(), RowIdentifier(0));
        assert_eq!(RowIdentifier::from_str("B").unwrap(), RowIdentifier(1));
        assert_eq!(RowIdentifier::from_str("C").unwrap(), RowIdentifier(2));
        assert_eq!(RowIdentifier::from_str("Z").unwrap(), RowIdentifier(25));
        assert_eq!(RowIdentifier::from_str("AA").unwrap(), RowIdentifier(26));

        assert_eq!(
            RowIdentifier::from_str("1").unwrap_err(),
            CoordinatesError::ParseError("1".to_string())
        );
    }

    #[test]
    fn test_coordinates_format() {
        assert_eq!(Coordinates::new(0, 0).to_string(), "A:1");
        assert_eq!(Coordinates::new(1, 50).to_string(), "B:51");
    }

    #[test]
    fn coordinates_iterator_up() {
        assert!(Coordinates::from_str("A:1")
            .unwrap()
            .iterator(super::Direction::Up)
            .next()
            .is_none());

        assert!(Coordinates::from_str("A:2")
            .unwrap()
            .iterator(super::Direction::Up)
            .next()
            .is_none());

        assert!(Coordinates::from_str("A:300")
            .unwrap()
            .iterator(super::Direction::Up)
            .next()
            .is_none());

        assert_eq!(
            Coordinates::from_str("B:1")
                .unwrap()
                .iterator(super::Direction::Up)
                .next()
                .unwrap(),
            Coordinates::from_str("A:1").unwrap()
        );

        assert_eq!(
            Coordinates::from_str("B:2")
                .unwrap()
                .iterator(super::Direction::Up)
                .next()
                .unwrap(),
            Coordinates::from_str("A:2").unwrap()
        );
    }

    #[test]
    fn coordinates_iterator_up_right() {
        assert!(Coordinates::from_str("A:1")
            .unwrap()
            .iterator(super::Direction::UpRight)
            .next()
            .is_none());

        assert!(Coordinates::from_str("A:100")
            .unwrap()
            .iterator(super::Direction::UpRight)
            .next()
            .is_none());

        assert_eq!(
            Coordinates::from_str("B:1")
                .unwrap()
                .iterator(super::Direction::UpRight)
                .next()
                .unwrap(),
            Coordinates::from_str("A:2").unwrap()
        );

        assert_eq!(
            Coordinates::from_str("E:30")
                .unwrap()
                .iterator(super::Direction::UpRight)
                .next()
                .unwrap(),
            Coordinates::from_str("D:31").unwrap()
        );
    }

    #[test]
    fn coordinates_iterator_right() {
        assert_eq!(
            Coordinates::from_str("A:1")
                .unwrap()
                .iterator(super::Direction::Right)
                .next()
                .unwrap(),
            Coordinates::from_str("A:2").unwrap()
        );

        assert_eq!(
            Coordinates::from_str("B:100")
                .unwrap()
                .iterator(super::Direction::Right)
                .next()
                .unwrap(),
            Coordinates::from_str("B:101").unwrap()
        );
    }

    #[test]
    fn coordinates_iterator_down_right() {
        assert_eq!(
            Coordinates::from_str("A:1")
                .unwrap()
                .iterator(super::Direction::DownRight)
                .next()
                .unwrap(),
            Coordinates::from_str("B:2").unwrap()
        );

        assert_eq!(
            Coordinates::from_str("B:100")
                .unwrap()
                .iterator(super::Direction::DownRight)
                .next()
                .unwrap(),
            Coordinates::from_str("C:101").unwrap()
        );

        assert_eq!(
            Coordinates::from_str("AA:200")
                .unwrap()
                .iterator(super::Direction::DownRight)
                .next()
                .unwrap(),
            Coordinates::from_str("AB:201").unwrap()
        );
    }

    #[test]
    fn coordinates_iterator_down() {
        assert_eq!(
            Coordinates::from_str("A:1")
                .unwrap()
                .iterator(super::Direction::Down)
                .next()
                .unwrap(),
            Coordinates::from_str("B:1").unwrap()
        );

        assert_eq!(
            Coordinates::from_str("B:100")
                .unwrap()
                .iterator(super::Direction::Down)
                .next()
                .unwrap(),
            Coordinates::from_str("C:100").unwrap()
        );

        assert_eq!(
            Coordinates::from_str("AA:200")
                .unwrap()
                .iterator(super::Direction::Down)
                .next()
                .unwrap(),
            Coordinates::from_str("AB:200").unwrap()
        );
    }

    #[test]
    fn coordinates_iterator_down_left() {
        assert!(Coordinates::from_str("A:1")
            .unwrap()
            .iterator(super::Direction::DownLeft)
            .next()
            .is_none());

        assert_eq!(
            Coordinates::from_str("A:2")
                .unwrap()
                .iterator(super::Direction::DownLeft)
                .next()
                .unwrap(),
            Coordinates::from_str("B:1").unwrap()
        );
        assert_eq!(
            Coordinates::from_str("B:100")
                .unwrap()
                .iterator(super::Direction::DownLeft)
                .next()
                .unwrap(),
            Coordinates::from_str("C:99").unwrap()
        );

        assert_eq!(
            Coordinates::from_str("AA:200")
                .unwrap()
                .iterator(super::Direction::DownLeft)
                .next()
                .unwrap(),
            Coordinates::from_str("AB:199").unwrap()
        );
    }

    #[test]
    fn coordinates_iterator_left() {
        assert!(Coordinates::from_str("A:1")
            .unwrap()
            .iterator(super::Direction::Left)
            .next()
            .is_none());

        assert_eq!(
            Coordinates::from_str("A:2")
                .unwrap()
                .iterator(super::Direction::Left)
                .next()
                .unwrap(),
            Coordinates::from_str("A:1").unwrap()
        );
        assert_eq!(
            Coordinates::from_str("B:100")
                .unwrap()
                .iterator(super::Direction::Left)
                .next()
                .unwrap(),
            Coordinates::from_str("B:99").unwrap()
        );

        assert_eq!(
            Coordinates::from_str("AA:200")
                .unwrap()
                .iterator(super::Direction::Left)
                .next()
                .unwrap(),
            Coordinates::from_str("AA:199").unwrap()
        );
    }

    #[test]
    fn coordinates_iterator_up_left() {
        assert!(Coordinates::from_str("A:1")
            .unwrap()
            .iterator(super::Direction::UpLeft)
            .next()
            .is_none());

        assert!(Coordinates::from_str("B:1")
            .unwrap()
            .iterator(super::Direction::UpLeft)
            .next()
            .is_none());

        assert!(Coordinates::from_str("A:1")
            .unwrap()
            .iterator(super::Direction::UpLeft)
            .next()
            .is_none());

        assert_eq!(
            Coordinates::from_str("B:100")
                .unwrap()
                .iterator(super::Direction::UpLeft)
                .next()
                .unwrap(),
            Coordinates::from_str("A:99").unwrap()
        );

        assert_eq!(
            Coordinates::from_str("AA:200")
                .unwrap()
                .iterator(super::Direction::UpLeft)
                .next()
                .unwrap(),
            Coordinates::from_str("Z:199").unwrap()
        );
    }
}
