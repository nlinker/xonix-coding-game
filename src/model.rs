
#![allow(unused)]

extern crate itertools;

use std::str::FromStr;

/// view
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Cell {
    Empty,
    Border,
    Owned(u8),
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Point(u16, u16);

enum Move {
    Right, Up, Left, Down, Stop
}

/// Field contains the information about the terrain
/// - `m` the number of rows
/// - `n` the number of cols
/// - `m×n` matrix of cells
struct Field {
    m: usize,
    n: usize,
    cells: Vec<Vec<Cell>>,
}

struct Stats {
    filled_count: u16,
    bite_count: u16,
    ouroboros_count: u16,
}

/// _pids_ is player ids, usually it is a database key
struct GameState {
    field: Field,
    pids: Vec<i32>,
    players: Vec<Vec<Point>>,
    origins: Vec<Point>,
    monster: Point
}

struct ParseError;

impl FromStr for GameState {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(ParseError)
    }
}

//#[stable(feature = "rust1", since = "1.0.0")]
//impl fmt::Display for Utf8Error {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        if let Some(error_len) = self.error_len {
//            write!(f, "invalid utf-8 sequence of {} bytes from index {}",
//                   error_len, self.valid_up_to)
//        } else {
//            write!(f, "incomplete utf-8 byte sequence from index {}", self.valid_up_to)
//        }
//    }
//}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_game_state() {
        let a = "a";
        assert_eq!("a", a)
    }
}
