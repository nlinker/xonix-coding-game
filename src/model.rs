
#![allow(unused)]

extern crate itertools;
extern crate core;

use std::str::FromStr;
use std::fmt;
use std::fmt::Formatter;
use std::error::Error;
use core::str;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

/// view
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Cell {
    Empty,
    Border,
    Owned(u8),
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Point(i16, i16);

pub enum Move {
    Right, Up, Left, Down, Stop,
}

/// Field contains the information about the terrain
/// - `m` the number of rows
/// - `n` the number of cols
/// - `m×n` matrix of cells
pub struct Field {
    m: usize,
    n: usize,
    cells: Vec<Vec<Cell>>,
}

/// Stats is updated on each step according to the things happened
pub struct Stats {
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
    reordering: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Player(pub Vec<Point>);

#[derive(Clone, Debug)]
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
        fn bound(x: i16, l: i16, r: i16) -> i16 {
            if x < l { l } else if r < x { r } else { x }
        }
        let neigh = vec![Point(0, -1), Point(-1, 0), Point(0, 1), Point(1, 0)];
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
        let mut layer0 = vec![vec![' ' as u8; n]; m];
        let mut layer1 = vec![vec!['.' as u8; n]; m];
        for i in 0..m {
            let cs = lines[i].as_bytes();
            for j in 0..(2 * n) {
                let c = cs[j];
                if j % 2 == 0 {
                    layer0[i][j / 2] = c;
                } else {
                    layer1[i][j / 2] = c;
                }
            }
        }
        let mut players_map: HashMap<u8, Vec<Point>> = HashMap::new();
        for i in 0..m {
            for j in 0..n {
                let c = layer1[i][j];
                if c.is_ascii_alphabetic() && c.is_ascii_uppercase() {
                    players_map.insert(c - ('A' as u8), vec![Point(i as i16, j as i16)]);
                }
            }
        }
        if let Some(&max_id) = players_map.keys().max() {
            for k in 0..=max_id {
                players_map.entry(k).or_insert_with(|| vec![]);
            }
        }
        let mut cells: Vec<Vec<Cell>> = vec![vec![Cell::Empty; n]; m];
        for i in 0..m {
            for j in 0..n {
                let c = layer0[i][j];
                let cell = if c == ('*' as u8) {
                    Cell::Border
                } else if ('0' as u8) < c && c < ('9' as u8) {
                    Cell::Owned(c - ('0' as u8))
                } else {
                    Cell::Empty
                };
                cells[i][j] = cell;
            }
        }
        // now build player bodies = tails + heads
        // head is the last element of the corresponding list
        for (k, mut body) in &mut players_map {
            if !body.is_empty() {
                // current point, start with the head
                let mut cp = Some(body[0]);
                let ct = ('a' as u8) + k; // the player's tail char
                while cp.is_some() {
                    // seek for lower letter around until not found
                    let t = cp.unwrap();
                    let point0 = neigh.iter().map(|Point(ni, nj)| {
                        let Point(mut i, mut j) = t;
                        i = bound(i + ni, 0, (m - 1) as i16);
                        j = bound(j + nj, 0, (n - 1) as i16);
                        Point(i, j)
                    }).filter(|p| {
                        let Point(pi, pj) = p;
                        !body.contains(p) && layer1[*pi as usize][*pj as usize] == ct
                    }).next();
                    if point0.is_some() {
                        body.insert(0, point0.unwrap());
                    }
                    cp = point0;
                }
            } else {
                //body is empty => do nothing, the player has been killed maybe
            }
        }
        let np = players_map.len();
        let mut filled_count = 0;
        let mut scores = vec![0u16; np];
        // calculate scores
        for i in 0..m {
            for j in 0..n {
                match cells[i][j] {
                    Cell::Empty => {}
                    Cell::Border => {
                        filled_count += 1;
                    }
                    Cell::Owned(k) => {
                        filled_count += 1;
                        scores[k as usize] = scores[k as usize] + 1;
                    }
                }
            }
        }
        // initialize origins by the number of players
        let mut players = vec![Player(vec![]); np];
        for k in 0..np {
            // to avoid
            let pts = players_map.remove(&(k as u8)).unwrap();
            players.push(Player(pts));
        }
        let field = Field { m, n, cells };
        // reordering and stats
        let triple = GameState::parse_string_rest(np, &rest);
        let reordering = triple.reordering.unwrap_or_else(|| GameState::create_default_permutation(np));
        let origins = triple.origins.unwrap_or_else(|| GameState::create_origins(m, n, np));
        let stats = triple.stats.unwrap_or_else(|| Stats {
            iteration: 0,
            filled_count,
            head_to_head_count: 0,
            ouroboros_count: 0,
            bite_count: 0,
            scores,
        });
        let player_names = (0..np).map(|i| format!("player-{}", i)).collect();
        let gs = GameState { field, player_names, players, origins, reordering };
        eprintln!("gs = {:?}", gs);
        return Err(ParseError)
    }

    fn parse_string_rest(np: usize, rest: &Vec<&str>) -> ParseRestResult {
        // TODO finish it
        ParseRestResult {
            reordering: None,
            origins: None,
            stats: None
        }
    }

    fn create_default_permutation(np: usize) -> Vec<u8> {
        (0..np).map(|x| x as u8).collect()
    }

    fn create_origins(m: usize, n: usize, np: usize) -> Vec<Point> {
        unreachable!()
    }
}

struct ParseRestResult {
    reordering: Option<Vec<u8>>,
    origins: Option<Vec<Point>>,
    stats: Option<Stats>,
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
