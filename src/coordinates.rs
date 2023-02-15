use regex::Regex;
use std::{
    fmt::Display,
    ops::{Add, Sub},
    str::FromStr,
};
use thiserror::Error;

use crate::{
    walker::{Walkable, Walker, WalkerIterator},
    Dir,
};

pub type Coords = Coordinates;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum CoordinatesError {
    #[error("Invalid coordinates format: '{0}'")]
    ParseError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinates {
    pub row: usize,
    pub col: usize,
}

impl Coordinates {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl Walkable for Coordinates {
    type WItem = Coordinates;
    type W = CoordinatesWalker;

    fn walker(&self, dir: Dir) -> Self::W {
        CoordinatesWalker(*self, dir)
    }
}

pub struct CoordinatesWalker(Coordinates, Dir);

impl IntoIterator for CoordinatesWalker {
    type Item = Coords;

    type IntoIter = WalkerIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        WalkerIterator::new(self.0, self.1)
    }
}

impl Walker for CoordinatesWalker {
    type WItem = Coordinates;

    fn walk(&self, length: usize) -> Option<Self::WItem> {
        match self.1 {
            Dir::Up => {
                if self.0.row >= length {
                    Some(self.0 - Coords::new(length, 0))
                } else {
                    None
                }
            }
            Dir::UpRight => {
                if self.0.row >= length {
                    Some(self.0 + Coordinates::new(0, length) - Coords::new(length, 0))
                } else {
                    None
                }
            }
            Dir::Right => Some(self.0 + Coords::new(0, length)),
            Dir::DownRight => Some(self.0 + Coords::new(length, length)),
            Dir::Down => Some(self.0 + Coords::new(length, 0)),
            Dir::DownLeft => {
                if self.0.col >= length {
                    Some(self.0 + Coords::new(length, 0) - Coords::new(0, length))
                } else {
                    None
                }
            }
            Dir::Left => {
                if self.0.col >= length {
                    Some(self.0 - Coords::new(0, length))
                } else {
                    None
                }
            }
            Dir::UpLeft => {
                if self.0.col >= length && self.0.row >= length {
                    Some(self.0 - Coords::new(length, length))
                } else {
                    None
                }
            }
        }
    }
}

impl Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Self) -> Self::Output {
        Coords::new(self.row + rhs.row, self.col + rhs.col)
    }
}

impl Sub for Coords {
    type Output = Coords;

    fn sub(self, rhs: Self) -> Self::Output {
        Coords::new(self.row - rhs.row, self.col - rhs.col)
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", RowNumber(self.row), self.col + 1)
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

        let row = RowNumber::from_str(
            captures
                .name("row")
                .expect("a capture with name row was expected")
                .as_str(),
        )?;

        Ok(Coords {
            row: row.into(),
            col: col - 1,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct RowNumber(usize);

impl RowNumber {
    pub fn new(number: usize) -> Self {
        RowNumber(number)
    }
}

impl From<RowNumber> for usize {
    fn from(value: RowNumber) -> Self {
        value.0
    }
}

impl FromStr for RowNumber {
    type Err = CoordinatesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valid_regex = Regex::new(r"\A([A-Z]|[a-z])+\z").expect("regex error");
        if !valid_regex.is_match(s) {
            return Err(CoordinatesError::ParseError(s.to_string()));
        }

        let mut res: usize = 0;
        for c in s.chars() {
            res *= 26;
            res += ((c.to_ascii_uppercase() as u8) - (b'A') + 1) as usize;
        }

        Ok(RowNumber(res - 1))
    }
}

impl Display for RowNumber {
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    use rstest::*;

    #[rstest]
    #[case("A", 0)]
    #[case("B", 1)]
    #[case("Z", 25)]
    #[case("AA", 26)]
    fn test_row_identifier_parse(#[case] input: &str, #[case] output: usize) {
        assert_eq!(RowNumber::from_str(input).unwrap(), RowNumber(output))
    }

    #[rstest]
    #[case("A", 0)]
    #[case("B", 1)]
    #[case("Z", 25)]
    #[case("AA", 26)]
    fn test_row_identifier_to_str(#[case] output: &str, #[case] input: usize) {
        assert_eq!(RowNumber(input).to_string(), output)
    }

    #[rstest]
    fn test_invalid_row_identifier_parse() {
        assert_eq!(
            RowNumber::from_str("1").unwrap_err(),
            CoordinatesError::ParseError("1".to_string())
        );

        assert_eq!(
            RowNumber::from_str(" / ").unwrap_err(),
            CoordinatesError::ParseError(" / ".to_string())
        );
    }

    #[test]
    fn test_coordinates_to_string() {
        assert_eq!(Coords::new(0, 0).to_string(), "A:1");
        assert_eq!(Coords::new(1, 50).to_string(), "B:51");
    }

    #[rstest]
    fn test_invalid_coordinates_from_str() {
        assert_eq!(
            Coords::from_str("B").unwrap_err(),
            CoordinatesError::ParseError("B".to_string())
        );

        assert_eq!(
            Coords::from_str("B1").unwrap_err(),
            CoordinatesError::ParseError("B1".to_string())
        );

        assert_eq!(
            Coords::from_str("B 1").unwrap_err(),
            CoordinatesError::ParseError("B 1".to_string())
        );
    }

    #[rstest]
    #[case("A:1", Coords{row: 0, col:0})]
    #[case("B:1", Coords{row: 1, col:0})]
    #[case("AA:26", Coords{row: 26, col:25})]
    fn test_coordinates_from_str(#[case] input: &str, #[case] output: Coords) {
        assert_eq!(Coords::from_str(input).unwrap(), output)
    }

    #[test]
    fn test_coordinates_iterator() {
        let c = Coords::from_str("A:1").unwrap();
        let mut up = c.walker(Dir::Up).into_iter();
        let mut up_right = c.walker(Dir::UpRight).into_iter();
        let mut right = c.walker(Dir::Right).into_iter();
        let mut down_right = c.walker(Dir::DownRight).into_iter();
        let mut down = c.walker(Dir::Down).into_iter();
        let mut down_left = c.walker(Dir::DownLeft).into_iter();
        let mut left = c.walker(Dir::Left).into_iter();
        let mut up_left = c.walker(Dir::UpLeft).into_iter();

        assert_eq!(up.next(), None);
        assert_eq!(up.next(), None);

        assert_eq!(up_right.next(), None);
        assert_eq!(up_right.next(), None);

        assert_eq!(right.next(), Some(Coords::from_str("A:2").unwrap()));
        assert_eq!(right.next(), Some(Coords::from_str("A:3").unwrap()));
        assert_eq!(right.next(), Some(Coords::from_str("A:4").unwrap()));

        assert_eq!(down_right.next(), Some(Coords::from_str("B:2").unwrap()));
        assert_eq!(down_right.next(), Some(Coords::from_str("C:3").unwrap()));

        assert_eq!(down.next(), Some(Coords::from_str("B:1").unwrap()));
        assert_eq!(down.next(), Some(Coords::from_str("C:1").unwrap()));

        assert_eq!(down_left.next(), None);
        assert_eq!(down_left.next(), None);

        assert_eq!(left.next(), None);
        assert_eq!(left.next(), None);

        assert_eq!(up_left.next(), None);
        assert_eq!(up_left.next(), None);

        let c = Coords::from_str("D:4").unwrap();
        let mut up = c.walker(Dir::Up).into_iter();
        let mut up_right = c.walker(Dir::UpRight).into_iter();
        let mut right = c.walker(Dir::Right).into_iter();
        let mut down_right = c.walker(Dir::DownRight).into_iter();
        let mut down = c.walker(Dir::Down).into_iter();
        let mut down_left = c.walker(Dir::DownLeft).into_iter();
        let mut left = c.walker(Dir::Left).into_iter();
        let mut up_left = c.walker(Dir::UpLeft).into_iter();

        assert_eq!(up.next(), Some(Coords::from_str("C:4").unwrap()));
        assert_eq!(up.next(), Some(Coords::from_str("B:4").unwrap()));

        assert_eq!(up_right.next(), Some(Coords::from_str("C:5").unwrap()));
        assert_eq!(up_right.next(), Some(Coords::from_str("B:6").unwrap()));

        assert_eq!(right.next(), Some(Coords::from_str("D:5").unwrap()));
        assert_eq!(right.next(), Some(Coords::from_str("D:6").unwrap()));
        assert_eq!(right.next(), Some(Coords::from_str("D:7").unwrap()));

        assert_eq!(down_right.next(), Some(Coords::from_str("E:5").unwrap()));
        assert_eq!(down_right.next(), Some(Coords::from_str("F:6").unwrap()));

        assert_eq!(down.next(), Some(Coords::from_str("E:4").unwrap()));
        assert_eq!(down.next(), Some(Coords::from_str("F:4").unwrap()));

        assert_eq!(down_left.next(), Some(Coords::from_str("E:3").unwrap()));
        assert_eq!(down_left.next(), Some(Coords::from_str("F:2").unwrap()));

        assert_eq!(left.next(), Some(Coords::from_str("D:3").unwrap()));
        assert_eq!(left.next(), Some(Coords::from_str("D:2").unwrap()));

        assert_eq!(up_left.next(), Some(Coords::from_str("C:3").unwrap()));
        assert_eq!(up_left.next(), Some(Coords::from_str("B:2").unwrap()));
    }
}
