use regex::Regex;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

type Coord = Coordinates;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum CoordinatesError {
    #[error("Invalid coordinates format: '{0}'")]
    ParseError(String),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CoordinatesIterator<'c> {
    coordinates: &'c Coordinates,
    direction: Direction,
    ix: usize,
}

impl<'c> Iterator for CoordinatesIterator<'c> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let x = match self.direction {
            Direction::Up => {
                if self.coordinates.row >= self.ix {
                    Some(Coord {
                        row: self.coordinates.row - self.ix,
                        col: self.coordinates.col,
                    })
                } else {
                    None
                }
            }
            Direction::UpRight => {
                if self.coordinates.row >= self.ix {
                    Some(Coord::new(
                        self.coordinates.row - self.ix,
                        self.coordinates.col + self.ix,
                    ))
                } else {
                    None
                }
            }

            Direction::Right => Some(Coord::new(
                self.coordinates.row,
                self.coordinates.col + self.ix,
            )),

            Direction::DownRight => Some(Coord::new(
                self.coordinates.row + self.ix,
                self.coordinates.col + self.ix,
            )),

            Direction::Down => Some(Coord::new(
                self.coordinates.row + self.ix,
                self.coordinates.col,
            )),

            Direction::DownLeft => {
                if self.coordinates.col >= self.ix {
                    Some(Coord::new(
                        self.coordinates.row + self.ix,
                        self.coordinates.col - self.ix,
                    ))
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.coordinates.col >= self.ix {
                    Some(Coord::new(
                        self.coordinates.row,
                        self.coordinates.col - self.ix,
                    ))
                } else {
                    None
                }
            }
            Direction::UpLeft => {
                if self.coordinates.col >= self.ix && self.coordinates.row >= self.ix {
                    Some(Coord::new(
                        self.coordinates.row - self.ix,
                        self.coordinates.col - self.ix,
                    ))
                } else {
                    None
                }
            }
        };
        self.ix = self.ix + 1;
        x
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
        CoordinatesIterator {
            coordinates: self,
            direction,
            ix: 1,
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", RowIdentifier::to_str(self.row), self.col + 1)
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

        let row = RowIdentifier::parse(
            captures
                .name("row")
                .expect("a capture with name row was expected")
                .as_str(),
        )?;

        Ok(Coordinates {
            row: row,
            col: col - 1,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RowIdentifier;

impl RowIdentifier {
    fn parse(str: &str) -> Result<usize, CoordinatesError> {
        let valid_regex = Regex::new(r"\A([A-Z]|[a-z])+\z").expect("regex error");
        if !valid_regex.is_match(str) {
            return Err(CoordinatesError::ParseError(str.to_string()));
        }

        let mut res: usize = 0;
        for c in str.chars() {
            res = res * 26;
            res = res + ((c.to_ascii_uppercase() as u8) - ('A' as u8) + 1) as usize;
        }

        Ok(res - 1)
    }

    fn to_str(row: usize) -> String {
        let mut id = String::new();
        let mut temp_row = row + 1;

        while temp_row > 0 {
            let letter = (temp_row - 1) % 26;
            id.push((letter + 65) as u8 as char);
            temp_row = (temp_row - 1) / 26;
        }

        id.chars().rev().collect::<String>()
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
        assert_eq!(RowIdentifier::parse(input).unwrap(), output)
    }

    #[rstest]
    #[case("A", 0)]
    #[case("B", 1)]
    #[case("Z", 25)]
    #[case("AA", 26)]
    fn test_row_identifier_to_str(#[case] output: &str, #[case] input: usize) {
        assert_eq!(RowIdentifier::to_str(input), output)
    }

    #[rstest]
    fn test_invalid_row_identifier_parse() {
        assert_eq!(
            RowIdentifier::parse("1").unwrap_err(),
            CoordinatesError::ParseError("1".to_string())
        );

        assert_eq!(
            RowIdentifier::parse(" / ").unwrap_err(),
            CoordinatesError::ParseError(" / ".to_string())
        );
    }

    #[test]
    fn test_coordinates_to_string() {
        assert_eq!(Coordinates::new(0, 0).to_string(), "A:1");
        assert_eq!(Coordinates::new(1, 50).to_string(), "B:51");
    }

    #[rstest]
    fn test_invalid_coordinates_from_str() {
        assert_eq!(
            Coordinates::from_str("B").unwrap_err(),
            CoordinatesError::ParseError("B".to_string())
        );

        assert_eq!(
            Coordinates::from_str("B1").unwrap_err(),
            CoordinatesError::ParseError("B1".to_string())
        );

        assert_eq!(
            Coordinates::from_str("B 1").unwrap_err(),
            CoordinatesError::ParseError("B 1".to_string())
        );
    }

    #[rstest]
    #[case("A:1", Coordinates{row: 0, col:0})]
    #[case("B:1", Coordinates{row: 1, col:0})]
    #[case("AA:26", Coordinates{row: 26, col:25})]
    fn test_coordinates_from_str(#[case] input: &str, #[case] output: Coordinates) {
        assert_eq!(Coordinates::from_str(input).unwrap(), output)
    }

    #[test]
    fn coordinates_iterator_up() {
        let c = Coordinates::from_str("A:1").unwrap();
        let mut i = c.iterator(Direction::Up);
        assert!(i.next().is_none());

        let c = Coordinates::from_str("A:2").unwrap();
        let mut i = c.iterator(Direction::Up);
        assert!(i.next().is_none());

        let c = Coordinates::from_str("B:1").unwrap();
        let mut i = c.iterator(Direction::Up);
        assert_eq!(i.next().unwrap(), Coordinates::from_str("A:1").unwrap());
        assert!(i.next().is_none());

        let c = Coordinates::from_str("C:3").unwrap();
        let mut i = c.iterator(Direction::Up);
        assert_eq!(i.next().unwrap(), Coordinates::from_str("B:3").unwrap());
        assert_eq!(i.next().unwrap(), Coordinates::from_str("A:3").unwrap());
        assert!(i.next().is_none());
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
