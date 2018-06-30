
#![allow(unused)]

extern crate itertools;

use std::str::FromStr;
use std::fmt;
use std::fmt::Formatter;
use std::error::Error;

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
    Right, Up, Left, Down, Stop,
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

/// Stats is updated on each step according to the things happened
struct Stats {
    iteration: u16,
    filled_count: u16,
    head_to_head_count: u16,
    ouroboros_count: u16,
    bite_count: u16,
    scores: Vec<u16>,
}

/// _player_names_ is player names
struct GameState {
    field: Field,
    player_names: Vec<String>,
    players: Vec<Player>,
    origins: Vec<Point>,
    // monsters: Vec<Point>,
    reordering: Vec<u8>,
}

struct Player(Vec<Point>);

#[derive(Debug)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "ParseError is here!") }
}

impl Error for ParseError {
    fn description(&self) -> &str { "Cannot parse the string to GameState" }
    fn cause(&self) -> Option<&Error> { None }
}

impl GameState {
    fn parse_string(s: &str) -> Result<GameState, ParseError> {
        return Err(ParseError)
    }
}

//struct ParseError;
//
//impl FromStr for GameState {
//    type Err = ParseError;
//    fn from_str(s: &str) -> Result<Self, Self::Err> {
//        Err(ParseError)
//    }
//}
//
//impl fmt::Display for GameState {
//    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//        unimplemented!()
//    }
//}

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
