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
use regex::{Regex, Match};
use itertools::free::join;
use std::fmt::Write;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cmp::Ordering;
use itertools::Itertools;

/// view
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Border,
    Owned(u8),
}

/// Note `Ord` defined below
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, PartialOrd)]
pub struct Point(pub i16, pub i16);

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Move {
    Right, Up, Left, Down, Stop,
}

/// Field contains the information about the terrain
/// - `m` the number of rows
/// - `n` the number of cols
/// - `m×n` matrix of cells
#[derive(Clone, Eq, PartialEq, Debug)]
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
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct GameState {
    pub field: Field,
    pub players: Vec<Player>,
    pub player_names: Vec<String>,
    pub origins: Vec<Point>,
    pub stats: Stats,
    pub reordering: Vec<u8>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Player(pub Vec<Point>);

#[derive(Clone, Debug)]
pub struct ParseError;

impl Player {
    fn head(&self) -> Option<&Point> {
        // the last element in the vector
        if self.0.is_empty() { None } else { Some(&self.0[self.0.len() - 1]) }
    }
    fn tail(&self) -> Option<&[Point]>{
        // all elements except the vector
        if self.0.is_empty() { None } else { Some(&self.0[0..self.0.len() - 1]) }
    }
    fn body(&self) -> &[Point] {
        &self.0[..]
    }
}

pub trait Bot {
    fn do_move(&mut self, idx: u8, gs: &GameState) -> Move;
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "ParseError is here!") }
}

impl Error for ParseError {
    fn description(&self) -> &str { "Cannot parse the string to GameState" }
    fn cause(&self) -> Option<&Error> { None }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if self.0 > other.0 {
            Ordering::Greater
        } else {
            self.1.cmp(&other.1)
        }
    }
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
                } else if ('0' as u8) <= c && c <= ('9' as u8) {
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
                    // seek for lowercase letter around the current point
                    // if something found, then add the point to the current body
                    // otherwise consider the body fully built
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
                // what to do if the body is empty?
                // so far just skip it
            }
        }

        let np = players_map.len();
        // calculate statistics
        let mut filled_count = 0;
        let mut scores = vec![0u16; np];
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

        let mut players = Vec::<Player>::with_capacity(np);
        for k in 0..np {
            // to avoid the error 'cannot borrow from indexed context'
            // we need to remove the bodies from players_map
            let pts = players_map.remove(&(k as u8)).unwrap();
            players.push(Player(pts));
        }
        let field = Field { m, n, cells };
        // parse reordering, origins and stats from the rest
        let triple = GameState::parse_string_rest(np, &rest)?;
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
        Ok(GameState { field, players, player_names, origins, stats, reordering })
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
                let caps1 = Regex::new("\\[(.*?)]").unwrap().captures(r);
                if caps1.is_some() {
                    let caps1 = caps1.unwrap();
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
            } else if l == "stats" {
                let caps1 = Regex::new("Stats\\((.*?)\\)").unwrap().captures(r);
                if caps1.is_some() {
                    let caps1 = caps1.unwrap().get(1).unwrap().as_str();
                    let caps2 = Regex::new("(\\d+),(\\d+),(\\d+),(\\d+),(\\d+),\\[(.*?)]").unwrap().captures(caps1);
                    if caps2.is_some() {
                        let c2 = caps2.unwrap();
                        let a1 = c2.get(1).unwrap().as_str().parse::<u16>().unwrap();
                        let a2 = c2.get(2).unwrap().as_str().parse::<u16>().unwrap();
                        let a3 = c2.get(3).unwrap().as_str().parse::<u16>().unwrap();
                        let a4 = c2.get(4).unwrap().as_str().parse::<u16>().unwrap();
                        let a5 = c2.get(5).unwrap().as_str().parse::<u16>().unwrap();
                        let scores: Vec<u16> = c2.get(6).unwrap().as_str()
                            .split(",")
                            .map(|s: &str| s.trim().parse::<u16>().unwrap())
                            .collect();
                        if scores.len() != np {
                            return Err(ParseError);
                        }
                        stats = Some(Stats {
                            iteration: a1,
                            filled_count: a2,
                            head_to_head_count: a3,
                            ouroboros_count: a4,
                            bite_count: a5,
                            scores,
                        });
                    }
                }
            } else if l == "origins" {
                let caps1 = Regex::new("\\[(.*?)]").unwrap().captures(r);
                if caps1.is_some() {
                    let caps1 = caps1.unwrap().get(1).unwrap().as_str();
                    let caps2 = Regex::new("\\((\\d+),(\\d+)\\),?").unwrap();
                    let mut list: Vec<Point> = vec![];
                    for c2 in caps2.captures_iter(caps1) {
                        let i = c2.get(1).unwrap().as_str().parse::<i16>().unwrap();
                        let j = c2.get(2).unwrap().as_str().parse::<i16>().unwrap();
                        list.push(Point(i, j))
                    }
                    if list.len() != np {
                        return Err(ParseError);
                    }
                    origins = Some(list);
                }
            }
        }
        Ok(ParseRestResult { reordering, origins, stats })
    }
}

//impl fmt::Debug for GameState {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        f.write_str(&self.field.m.to_string());
//        f.write_str(&self.field.n.to_string());
//        Ok(())
//    }
//}

impl fmt::Display for Point {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_char('(');
        fmt.write_str(&self.0.to_string());
        fmt.write_char(',');
        fmt.write_str(&self.1.to_string());
        fmt.write_char(')');
        Ok(())
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let m = self.field.m;
        let n = self.field.n;
        let np = self.players.len();

        let mut layer0 = vec![vec![' ' as u8; n]; m];
        let mut layer1 = vec![vec!['.' as u8; n]; m];
        for i in 0..m {
            for j in 0..n {
                let cell = self.field.cells[i][j];
                match cell {
                    Cell::Empty => {layer0[i][j] = ' ' as u8}
                    Cell::Border => {layer0[i][j] = '*' as u8}
                    Cell::Owned(c) => layer0[i][j] = ('0' as u8) + c,
                }
            }
        }
        for k in 0..np {
            let player = &self.players[k].0;
            let ch = ('A' as u8) + (k as u8);
            for l in 0..player.len() {
                let i = player[l].0 as usize;
                let j = player[l].1 as usize;
                // if it is the last element == player's head
                if l == player.len() - 1 {
                    layer1[i][j] = ch;
                } else {
                    layer1[i][j] = ch.to_ascii_lowercase();
                }
            }
        }
        // now put all the stuff
        for i in 0..m {
            for j in 0..n {
                fmt.write_char(layer0[i][j] as char);
                fmt.write_char(layer1[i][j] as char);
            }
            fmt.write_char('\n');
        }
        fmt.write_str("reordering=[");
        fmt.write_str(&join(&self.reordering[..], &","));
        fmt.write_str("]\n");

        fmt.write_str("stats=Stats(");
        fmt.write_str(&format!("{},{},{},{},{},[",
            &self.stats.iteration,
            &self.stats.filled_count,
            &self.stats.head_to_head_count,
            &self.stats.ouroboros_count,
            &self.stats.bite_count
        ));
        fmt.write_str(&join(&self.stats.scores[..], &","));
        fmt.write_str("])\n");

        fmt.write_str("origins=[");
        fmt.write_str(&join(&self.origins[..], &","));
        fmt.write_str("]");

        Ok(())
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
            vec![]
        }
        1 => {
            let p0 = index_of(&perm, 0);
            vec![corners[p0]]
        }
        2 => {
            let p0 = index_of(&perm, 0);
            let p1 = index_of(&perm, 1);
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

pub fn flood(field: &Field, boundary: &HashSet<Point>, start: Point) -> HashSet<Point> {
    let m = field.m as i16;
    let n = field.n as i16;
    let neighbors = vec![Point(0, -1), Point(-1, 0), Point(0, 1), Point(1, 0)];

    // result is the growing set of points describing the filled area
    let mut result: HashSet<Point> = HashSet::new();

    let has_inside = |Point(i, j)| {
        0 <= i && i < m && 0 <= j && j < n
    };
    let in_area = |p, result: &HashSet<Point>| {
        has_inside(p)
            && !result.contains(&p)
            && !boundary.contains(&p)
            && field.cells[p.0 as usize][p.1 as usize] == Cell::Empty
    };
    // if the starting point on the boundary, return immediately
    if !in_area(start, &result) {
        return result;
    }
    // if the starting point is somewhere inside, calculate the area
    let mut queue: VecDeque<Point> = VecDeque::with_capacity(field.m + field.n);
    queue.push_back(start);
    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        result.insert(cur);
        let mut candidates = neighbors.iter()
            .map(|p| Point(cur.0 + p.0, cur.1 + p.1))
            .filter(|p| in_area(*p, &result) && !queue.contains(p))
            .collect();
        queue.append(&mut candidates);
    }
    result
}

pub fn calculate_flood_area(field: &Field, body: &Vec<Point>) -> Vec<Point> {
    let neighbors = vec![Point(0, -1), Point(-1, 0), Point(0, 1), Point(1, 0)];
    let boundary: HashSet<Point> = body.iter().cloned().collect();
    let mut areas: Vec<HashSet<Point>> = vec![];

    let in_areas: Box<Fn(Point, &Vec<HashSet<Point>>) -> bool> = Box::new(|p, areas| {
        areas.iter().any(|ps| ps.contains(&p))
    });

    for b in body.iter() {
        // search in the neighborhood of p empty areas
        // empty means not only empty surface but also free of players
        let mut starts: Vec<Point> = neighbors.iter()
            .map(|p| Point(b.0 + p.0, b.1 + p.1))
            .filter(|p| has_inside(&field, *p)
                && field.cells[p.0 as usize][p.1 as usize] == Cell::Empty
                && !boundary.contains(p)
                && !in_areas(*p, &areas))
            .collect();
        for sp in starts {
            if !in_areas(sp, &areas) {
                let flood = flood(&field, &boundary, sp);
                areas.push(flood);
            }
        }
    }
    // to handle the case, add a phony empty area
    // * A * * * B
    // * a       *
    // *         *
    // D * * * * C
    if areas.len() <= 1 {
        areas.push(HashSet::new());
    }
    // seek for the area by the minimum size
    let mut flooded: Vec<Point> = areas.iter()
        .min_by(|s1, s2| s1.len().cmp(&s2.len())).unwrap()
        .iter()
        .map(|p| *p)
        .collect_vec();
    flooded.append(&mut body.clone());
    flooded
}

pub fn step(gs: &mut GameState, idx: u8, mv: Move) {
    let index = idx as usize;
    let np = gs.players.len();

    let old_head = *gs.players[index].head().unwrap(); // XXX handle broken invariant
    let new_head = calculate_head(&gs.field, old_head, mv);
    let cells = &gs.field.cells;
    // let mut stats = &gs.stats;

    // the player hasn't effectively moved
    if old_head == new_head {
        return;
    }
    let old_cell = cells[old_head.0 as usize][old_head.1 as usize];
    let new_cell = cells[new_head.0 as usize][new_head.1 as usize];
    // detect a collision
    let collision = (0..np)
        .filter(|k| gs.players[*k].body().contains(&new_head))
        .map(|k| (k as u8, &gs.players[k]))
        .collect::<Vec<(u8, &Player)>>();
    if new_head != old_head && !collision.is_empty() {
        let (coll_idx, coll_player) = collision.first().unwrap();
        let coll_head = *coll_player.head().unwrap(); // XXX handle broken invariant
        if new_head == coll_head {
            // the player bumps with the other player's head
//            gs.stats.head_to_head_count += 1;
        } else if *coll_idx == idx {
            // the player eats itself
//            respawn_player(gs, idx);
//            gs.stats.ouroboros_count += 1;
        } else {
            // the player `index` moves, and other player `coll_idx` dies,
            // if the current player was on the empty cell, its tail increases
            // otherwise it just moves to the next cell
//            respawn_player(gs, *coll_idx);
//            gs.stats.bite_count += 1;
//            if old_cell == Cell::Empty {
//                &gs.players[index].0.push(new_head);
//            } else {
//                &gs.players[index].0.clear();
//                &gs.players[index].0.push(new_head);
//            }
            eprintln!("collision = {:?}", collision);
        }
    } else if new_head != old_head && old_cell != Cell::Empty {

    } else if new_head != old_head && new_cell != Cell::Empty {

    } else if new_head != old_head {

    }
}

fn calculate_head(field: &Field, old_p: Point, mv: Move) -> Point {
    let Point(i, j) = old_p;
    let (di, dj) = match mv {
        Move::Right => (0, 1),
        Move::Up    => (-1, 0),
        Move::Left  => (0, -1),
        Move::Down  => (1, 0),
        Move::Stop  => (0, 0),
    };
    let new_p = Point(i + di, j + dj);
    if has_inside(&field, new_p) { new_p } else { old_p }
}

fn respawn_player(gs: &mut GameState, idx: u8) {

}

/*
    fn calculate_head(field: &Field, p: Point, Move move) -> Point {
        int row = bot.getRow();
        int col = bot.getCol();
        switch (move) {
            case UP:
                row -= (0 <= row - 1) ? 1 : 0;
                break;
            case DOWN:
                row += (row + 1 < field.getHeight()) ? 1 : 0;
                break;
            case LEFT:
                col -= (0 <= col - 1) ? 1 : 0;
                break;
            case RIGHT:
                col += (col + 1 < field.getWidth()) ? 1 : 0;
                break;
            case STOP: // stay at position
                break;
        }
        return Point.of(row, col);
    }


*/


fn has_inside(field: &Field, p: Point) -> bool {
    let Point(i, j) = p;
    0 <= i && i < (field.m as i16) && 0 <= j && j < (field.n as i16)
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ParseRestResult {
    reordering: Option<Vec<u8>>,
    origins: Option<Vec<Point>>,
    stats: Option<Stats>,
}


//struct ParseError;
//
//impl FromStr for GameState {
//    type Err = ParseError;
//    fn from_str(s: &str) -> Result<Self, Self::Err> {
//        Err(ParseError)
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
