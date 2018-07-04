#![allow(unused)]

extern crate core;
extern crate itertools;
extern crate regex;

use std::str::FromStr;
use std::fmt;
use std::fmt::Formatter;
use std::error::Error;
use core::str;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use rand::prelude::{Rng, RngCore};
use rand::isaac::IsaacRng;
use regex::Regex;

/// view
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Border,
    Owned(u8),
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Point(pub i16, pub i16);

pub enum Move {
    Right, Up, Left, Down, Stop,
}

/// Field contains the information about the terrain
/// - `m` the number of rows
/// - `n` the number of cols
/// - `m×n` matrix of cells
#[derive(Eq, PartialEq, Debug)]
pub struct Field {
    pub m: usize,
    pub n: usize,
    pub cells: Vec<Vec<Cell>>,
}

/// Stats is updated on each step according to the things happened
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Stats {
    pub iteration: u16,
    pub filled_count: u16,
    pub head_to_head_count: u16,
    pub ouroboros_count: u16,
    pub bite_count: u16,
    pub scores: Vec<u16>,
}

/// _player_names_ is player names
pub struct GameState {
    pub field: Field,
    pub player_names: Vec<String>,
    pub players: Vec<Player>,
    pub origins: Vec<Point>,
    pub reordering: Vec<u8>,
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
        let triple = GameState::parse_string_rest(np, &rest).unwrap();
        let reordering = triple.reordering.unwrap_or_else(|| create_default_permutation(np));
        let origins = triple.origins.unwrap_or_else(|| create_origins_n(m, n, np));
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

    pub fn parse_string_rest(np: usize, rest: &Vec<&str>) -> Result<ParseRestResult, ParseError> {
        let mut reordering: Option<Vec<u8>> = None;
        let mut origins: Option<Vec<Point>> = None;
        let mut stats: Option<Stats> = None;
        for s in rest {
            let mut lr = s.split("=");
            let l = lr.next().unwrap().trim();
            let r = lr.next().unwrap().trim();
            if l == "reordering" {
                let caps10 = Regex::new("\\[(.*?)]").unwrap().captures(r);
                if caps10.is_some() {
                    let caps1 = caps10.unwrap();
                    let list: Vec<u8> = caps1.get(1).unwrap().as_str()
                        .split(",")
                        .map(|s: &str| s.trim().parse::<u8>().unwrap())
                        .collect();
                    // check
                    let all_present = (0..np as u8).all(|x| list.contains(&x));
                    if list.len() != np || !all_present {
                        return Err(ParseError);
                    }
                    reordering = Some(list);
                }
            } else if l == "origins" {

            }

        }
        Ok(ParseRestResult { reordering, origins, stats })
    }
/*
    private static ParseRestResult parseRest(int np, List<String> rest) {
        Optional<List<Integer>> reordering = Optional.empty();
        Optional<List<Point>> origins = Optional.empty();
        Optional<Stats> stats = Optional.empty();
        for (String str : rest) {
            String[] lr = str.split("=");
            String l = lr[0].trim();
            String r = lr[1].trim();
            switch (l) {
                case "reordering": {
                    Matcher m1 = Pattern.compile("\\[(.*?)]").matcher(r);
                    if (m1.matches()) {
                        List<Integer> list = Arrays.stream(m1.group(1).split(","))
                            .map(s -> Integer.parseInt(s.trim()))
                            .collect(toCollection(ArrayList::new));
                        // check
                        boolean allPresent = IntStream.range(0, np).allMatch(list::contains);
                        if (list.size() != np || !allPresent) {
                            throw new RuntimeException("Cannot parse, np=" + np + " rest=" + rest);
                        }
                        reordering = Optional.of(list);
                    }
                    break;
                }
                case "stats": {
                    Matcher m1 = Pattern.compile("Stats\\((.*?)\\)").matcher(r);
                    if (m1.matches()) {
                        Matcher m2 = Pattern.compile("(\\d+),(\\d+),(\\d+),(\\d+),(\\d+),\\[(.*?)]")
                            .matcher(m1.group(1));
                        if (m2.matches()) {
                            int a1 = Integer.parseInt(m2.group(1));
                            int a2 = Integer.parseInt(m2.group(2));
                            int a3 = Integer.parseInt(m2.group(3));
                            int a4 = Integer.parseInt(m2.group(4));
                            int a5 = Integer.parseInt(m2.group(5));
                            List<Integer> scores = Arrays.stream(m2.group(6).split(","))
                                .map(s -> Integer.parseInt(s.trim()))
                                .collect(toCollection(ArrayList::new));
                            stats = Optional.of(new Stats(a1, a2, a3, a4, a5, scores));
                        }
                    }
                    break;
                }
                case "origins": {
                    Matcher m1 = Pattern.compile("\\[(.*?)]").matcher(r);
                    if (m1.matches()) {
                        Matcher m2 = Pattern.compile("\\((\\d+),(\\d+)\\),?").matcher(m1.group(1));
                        List<Point> list = new ArrayList<>();
                        while (m2.find()) {
                            list.add(Point.of(
                                Integer.parseInt(m2.group(1)),
                                Integer.parseInt(m2.group(2))
                            ));
                        }
                        if (list.size() != np) {
                            throw new RuntimeException("Cannot parse, np=" + np + " rest=" + rest);
                        }
                        origins = Optional.of(list);
                    }
                    break;
                }
                default:
                    throw new RuntimeException("Unexpected key: " + l);
            }
        }
        return new ParseRestResult(reordering, origins, stats);
    }
*/

    fn to_string(&self) -> String {
        unimplemented!()
    }
}

pub fn create_default_permutation(np: usize) -> Vec<u8> {
    (0..np).map(|x| x as u8).collect()
}

pub fn copy_shuffled_permutation(xs: &Vec<u8>, random: &mut RngCore) -> Vec<u8> {
    let mut tmp = xs.clone();
    random.shuffle(tmp.as_mut_slice());
    return tmp;
}

pub fn create_origins_n(height: usize, width: usize, np: usize) -> Vec<Point> {
    let perm = create_default_permutation(np);
    create_origins(height, width, perm)
}

pub fn create_origins(height: usize, width: usize, perm: Vec<u8>) -> Vec<Point> {
    let m = height as i16;
    let n = width as i16;
    let b2p: Box<Fn(usize) -> Point> = Box::new(move |l| border_to_point(height, width, l));
    fn index_of(xs: &Vec<u8>, x: u8) -> usize {
        xs.iter().position(|&z| { z == x }).unwrap()
    }
    let np = perm.len();
    let corners = vec![Point(0, 0), Point(m - 1, n - 1), Point(0, n - 1), Point(m - 1, 0)];
    match np {
        0 => {
            eprintln!("corners = {:?}", corners);
            vec![]
        }
        1 => {
            let p0 = index_of(&perm, 0);
            vec![corners[p0]]
        }
        2 => {
            let p0 = index_of(&perm, 0);
            let p1 = index_of(&perm, 1);
            eprintln!("perm, p0, p1 = {:?} {:#?} {:#?}", perm, p0, p1);
            vec![corners[p0], corners[p1]]
        }
        3 => {
            let p0 = index_of(&perm, 0);
            let p1 = index_of(&perm, 1);
            let p2 = index_of(&perm, 2);
            vec![corners[p0], corners[p1], corners[p2]]
        }
        4 => {
            let p0 = index_of(&perm, 0);
            let p1 = index_of(&perm, 1);
            let p2 = index_of(&perm, 2);
            let p3 = index_of(&perm, 3);
            vec![corners[p0], corners[p1], corners[p2], corners[p3]]
        }
        _ => {
            // uniformly distribute across the perimeter
            let step: usize = 2 * (height + width - 2) / np;
            let mut opts: Vec<Option<Point>> = vec![None; np];
            for k in 0..np {
                opts[index_of(&perm, k as u8)] = Some(b2p(k * step));
            }
            opts.iter().map(|opt| opt.unwrap()).collect()
        }
    }
}

pub fn create_default_field(m: usize, n: usize) -> Field {
    let mut cells: Vec<Vec<Cell>> = vec![vec![Cell::Empty; n]; m];
    for i in 0..m {
        for j in 0..n {
            cells[i][j] = if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                Cell::Border
            } else {
                Cell::Empty
            }
        }
    }
    Field { m, n, cells }
}

pub fn border_to_point(height: usize, width: usize, pos: usize) -> Point {
    let m = height as i16;
    let n = width as i16;
    let pos = (pos as i16) % (2 * (m + n) - 4);
    if pos < n {
        Point(0, pos)
    } else if pos < n + m - 2 {
        Point(pos - n + 1, n - 1)
    } else if pos < n + n + m - 2 {
        return Point(m - 1, n + n + m - 3 - pos)
    } else {
        return Point(n + n + m + m - 4 - pos, 0)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ParseRestResult {
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
