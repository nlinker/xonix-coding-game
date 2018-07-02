
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
pub struct GameState {
    field: Field,
    player_names: Vec<String>,
    players: Vec<Player>,
    origins: Vec<Point>,
    // monsters: Vec<Point>,
    reordering: Vec<u8>,
}

struct Player(Vec<Point>);

#[derive(Debug)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "ParseError is here!") }
}

impl Error for ParseError {
    fn description(&self) -> &str { "Cannot parse the string to GameState" }
    fn cause(&self) -> Option<&Error> { None }
}

impl GameState {
    pub fn parse_string(str: &str) -> Result<GameState, ParseError> {
        // detect sizes
        let raw_lines: Vec<&str> = str.split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        let mut lines: Vec<&str> = Vec::with_capacity(raw_lines.len());
        let mut rest: Vec<&str> = Vec::new();
        for s in raw_lines {
            if s.starts_with("*") {
                lines.push(s);
            } else {
                rest.push(s);
            }
        }
        let m = lines.len();
        let n = lines.iter().map(|it| it.len() / 2).max().unwrap_or(0);
//        for c in lines[0].chars() {
//            let () = c;
//        };
        let mut layer0 = vec![vec![' '; n]; m];
        let mut layer1 = vec![vec!['.'; n]; m];
        for i in 0..m {
            let mut cs = lines[i].chars();
            for j in 0..(2 * n) {
                let c0 = cs.nth(j);
                print!("{}:{:?} ", j, c0);
                let c = c0.unwrap_or('#');
                if j % 2 == 0 {
                    layer0[i][j / 2] = c;
                } else {
                    layer1[i][j / 2] = c;
                }
            }
        }

        eprintln!("lines = {:?}", lines);
        eprintln!("layer0 = {:?}", layer0);
        eprintln!("layer1 = {:?}", layer1);
        eprintln!("m, n = {:#?}, {:#?}", m, n);

        return Err(ParseError)
    }
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.field.m.to_string());
        f.write_str(&self.field.n.to_string());
        Ok(())
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
