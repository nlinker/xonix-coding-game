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
use rand::prelude::{Rng, RngCore, FromEntropy};
use rand::isaac::IsaacRng;
use regex::{Regex, Match as RegexMatch};
use itertools::free::join;
use std::fmt::Write;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cmp::Ordering;
use itertools::Itertools;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::RefCell;

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
    pub iteration: u32,
    pub filled_count: u16,
    pub head_to_head_count: u16,
    pub ouroboros_count: u16,
    pub bite_count: u16,
    pub scores: Vec<u16>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Player(pub Vec<Point>);

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

#[derive(Debug)]
pub struct Match {
    pub duration: u32,
    pub ratio: f32,
    pub game_state: GameState,
    pub random_seed: Option<u64>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Replay {
    pub height: usize,
    pub width: usize,
    pub duration: u32,
    pub ratio: f32,
    pub player_names: Vec<String>,
    pub moves: Vec<Vec<Move>>,
    pub random_seed: Option<u64>,
}

//#[derive(Clone, Eq, PartialEq, Debug)]
//pub struct ClientGameState { pub field: Field }
//
//#[derive(Clone, Eq, PartialEq, Debug)]
//pub struct ClientGameStateDelta {}

#[derive(Clone, Debug)]
pub struct ParseError;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ParseRestResult {
    reordering: Option<Vec<u8>>,
    origins: Option<Vec<Point>>,
    stats: Option<Stats>,
}

impl Player {
    fn head(&self) -> Option<&Point> {
        // the last element in the vector
        if self.0.is_empty() { None } else { Some(&self.0[self.0.len() - 1]) }
    }
    fn tail(&self) -> Option<&[Point]>{
        // all elements except the vector
        if self.0.is_empty() { None } else { Some(&self.0[0..self.0.len() - 1]) }
    }
    fn body(&self) -> &Vec<Point> {
        &self.0
    }
    fn body_mut(&mut self) -> &mut Vec<Point> {
        &mut self.0
    }
}

pub trait Bot {
    // the bot is mutable
    fn reset(&mut self, gs: &GameState, idx: u8, seed: u64);
    fn do_move(&mut self, gs: &GameState) -> Move;
}

impl fmt::Debug for Bot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "bot") }
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
        let player_names = (0..np).map(|i| ((('A' as u8) + (i as u8)) as char).to_string()).collect();
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
                        let a1 = c2.get(1).unwrap().as_str().parse::<u32>().unwrap();
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

    pub fn format_string(&self, field_only: bool) -> String {
        let m = self.field.m;
        let n = self.field.n;
        let np = self.players.len();
        let capacity = if field_only { m * (2 * n + 1) + 2 } else { m * 2 * (m + n) + 10 * np + 30 };
        let mut result = String::with_capacity(capacity);

        let mut layer0 = vec![vec![' ' as u8; n]; m];
        let mut layer1 = vec![vec!['.' as u8; n]; m];
        for i in 0..m {
            for j in 0..n {
                let cell = self.field.cells[i][j];
                match cell {
                    Cell::Empty => { layer0[i][j] = ' ' as u8 }
                    Cell::Border => { layer0[i][j] = '*' as u8 }
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
                result.push(layer0[i][j] as char);
                result.push(layer1[i][j] as char);
            }
            result.push('\n');
        }
        if !field_only {
            result.push_str("reordering=[");
            result.push_str(&join(&self.reordering[..], &","));
            result.push_str("]\n");

            result.push_str("stats=Stats(");
            result.push_str(&format!("{},{},{},{},{},[",
                &self.stats.iteration,
                &self.stats.filled_count,
                &self.stats.head_to_head_count,
                &self.stats.ouroboros_count,
                &self.stats.bite_count
            ));
            result.push_str(&join(&self.stats.scores[..], &","));
            result.push_str("])\n");

            result.push_str("origins=[");
            result.push_str(&join(&self.origins[..], &","));
            result.push_str("]");
        }
        return result;
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

impl FromStr for GameState {
    type Err = ParseError;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        GameState::parse_string(str)
    }
}

//impl fmt::Debug for GameState {
//    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//        fmt.write_str(&format!("{}", self))
//    }
//}

impl fmt::Display for GameState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.format_string(false));
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
    create_origins(height, width, &perm)
}

pub fn create_origins(height: usize, width: usize, perm: &Vec<u8>) -> Vec<Point> {
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

pub fn create_default_field(height: usize, width: usize) -> Field {
    let m = height;
    let n = width;
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

    let old_head = *gs.players[index].head().expect("Broken invariant");
    let new_head = calculate_head(&gs.field, old_head, mv);
    // let mut stats = &gs.stats;

    // the player hasn't effectively moved
    if old_head == new_head {
        return;
    }
    let old_cell = gs.field.cells[old_head.0 as usize][old_head.1 as usize];
    let new_cell = gs.field.cells[new_head.0 as usize][new_head.1 as usize];
    // detect a collision
    let collision = (0..np).filter(|k| gs.players[*k].body().contains(&new_head)).next();
    if new_head != old_head && collision.is_some() {
        let coll_idx = collision.unwrap();
        let coll_head = *gs.players[coll_idx].head().expect("Broken invariant");
        if new_head == coll_head {
            // the player bumps with the other player's head
            gs.stats.head_to_head_count += 1;
        } else if coll_idx == index {
            // the player eats itself
            let respawn = calculate_respawn(gs, index).expect("Broken invariant");
            gs.players[index].body_mut().clear();
            gs.players[index].body_mut().push(respawn);
            gs.stats.ouroboros_count += 1;
        } else {
            // the player `index` moves, and other player `coll_idx` dies,
            // if the current player was on the empty cell, its tail increases
            // otherwise it just moves to the next cell
            let respawn = calculate_respawn(gs, coll_idx).expect("Broken invariant");
            gs.players[coll_idx].body_mut().clear();
            gs.players[coll_idx].body_mut().push(respawn);
            gs.stats.bite_count += 1;
            if old_cell == Cell::Empty {
                gs.players[index].body_mut().push(new_head);
            } else {
                gs.players[index].body_mut().clear();
                gs.players[index].body_mut().push(new_head);
            }
        }
    } else if new_head != old_head && old_cell != Cell::Empty {
        // we stay on the nonempty cell
        // single head, don't make the tail, just setPoint the head
        // otherwise we should have made the contour from the previous step
        if gs.players[index].body().len() > 1 {
            panic!("Broken invariant");
        }
        gs.players[index].body_mut().clear();
        gs.players[index].body_mut().push(new_head);
    } else if new_head != old_head && new_cell != Cell::Empty {
        // we step from empty to nonempty, calculate the contours
        // flood area now becomes owned by the current player
        // flood removes tails if any
        let flooded = calculate_flood_area(&gs.field, gs.players[index].body());
        for p in &flooded {
            let i = p.0 as usize;
            let j = p.1 as usize;
            gs.field.cells[i][j] = Cell::Owned(index as u8);
        }
        for k in 0..np {
            if k == index {
                gs.players[k].body_mut().clear();
                gs.players[k].body_mut().push(new_head);
            } else if gs.players[k].body().is_empty() {
                panic!("Broken invariant");
            } else {
                let head = *gs.players[k].head().expect("Broken invariant");
                let mut rest = gs.players[k].body().iter().filter(|p| {
                    // if the cell is Owned
                    let i = p.0 as usize;
                    let j = p.1 as usize;
                    match gs.field.cells[i][j] {
                        Cell::Owned(_) => true,
                        _ => false
                    }})
                    .map(|p| *p)
                    .collect_vec();
                // build the k-th body
                if rest.contains(&head) {
                    gs.players[k].body_mut().clear();
                    gs.players[k].body_mut().push(head);
                } else {
                    gs.players[k].body_mut().clear();
                    gs.players[k].body_mut().append(&mut rest);
                    gs.players[k].body_mut().push(head);
                }
            }
        }
        // finally update statistics
        gs.stats.scores[index] += (&flooded).len() as u16;
        gs.stats.filled_count += (&flooded).len() as u16;
    } else if new_head != old_head {
        // old_cell == Empty && new_cell == Empty (for sure)
        // we step into empty area, increase the tail
        // (head is the last element)
        gs.players[index].body_mut().push(new_head);
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

pub fn calculate_respawn(gs: &GameState, dead_idx: usize) -> Option<Point> {
    let np = gs.players.len();
    let mut others = HashSet::new();
    for k in 0..np {
        if k != dead_idx {
            gs.players[k].body().iter().foreach(|p| { others.insert(p); })
        }
    }
    let is_accessible = |p: Point| {
        let i = p.0 as usize;
        let j = p.1 as usize;
        has_inside(&gs.field, p) && gs.field.cells[i][j] == Cell::Border && !others.contains(&p)
    };

    // find the closest to the origin nonempty cell
    let origin = gs.origins[dead_idx];
    if is_accessible(origin) {
        return Some(origin);
    }
    for r in 1..((gs.field.m + gs.field.n) as i16) {
        for k in 0..r {
            let p1 = Point(origin.0 - k, origin.1 + r - k);
            let p2 = Point(origin.0 - r + k, origin.1 - k);
            let p3 = Point(origin.0 + k, origin.1 - r + k);
            let p4 = Point(origin.0 + r - k, origin.1 + k);
            if is_accessible(p1) { return Some(p1) };
            if is_accessible(p2) { return Some(p2) };
            if is_accessible(p3) { return Some(p3) };
            if is_accessible(p4) { return Some(p4) };
        }
    }
    None
}

fn has_inside(field: &Field, p: Point) -> bool {
    let Point(i, j) = p;
    0 <= i && i < (field.m as i16) && 0 <= j && j < (field.n as i16)
}

pub fn create_match<T: AsRef<str>>(
    height: usize, width: usize, player_names: &[T], duration: u32, ratio: f32,
    random_seed: Option<u64>
) -> Match {
    let np = player_names.len();
    let mut initializer_rng = random_seed.map(|seed| IsaacRng::new_from_u64(seed));
    let field = create_default_field(height, width);
    let perm0 = create_default_permutation(np);
    let origin_perm = match initializer_rng.borrow_mut() {
        Some(ref mut r) => copy_shuffled_permutation(&perm0, r),
        None => perm0.clone(),
    };
    let reordering = match initializer_rng.borrow_mut() {
        Some(ref mut r) => copy_shuffled_permutation(&perm0, r),
        None => perm0.clone()
    };
    // permute players if we have random generator
    let origins = create_origins(height, width, &origin_perm);
    let players = origins.iter().map(|&o| Player(vec![o])).collect();
    let player_names = player_names.iter().map(|s| s.as_ref().to_owned()).collect();
    let mut filled_count = 0;
    let mut scores = vec![0u16; np];
    for i in 0..height {
        for j in 0..width {
            match field.cells[i][j] {
                Cell::Empty => {}
                Cell::Border => { filled_count += 1; }
                Cell::Owned(k) => {
                    filled_count += 1;
                    scores[k as usize] = scores[k as usize] + 1;
                }
            }
        }
    }
    let stats = Stats{
        iteration: 0,
        filled_count,
        head_to_head_count: 0,
        ouroboros_count: 0,
        bite_count: 0,
        scores,
    };
    let game_state = GameState { field, players, player_names, origins, stats, reordering };
    Match { duration, ratio, game_state, random_seed }
}

pub fn run_match<B: Bot>(the_match: &mut Match, bots: &mut [B], logger: &Fn(&GameState)) -> Replay {
    let nb = bots.len();
    debug_assert_eq!(nb, the_match.game_state.reordering.len());
    debug_assert_eq!(nb, the_match.game_state.players.len());
    debug_assert_eq!(nb, the_match.game_state.player_names.len());
    let mut all_moves: Vec<Vec<Move>> = Vec::with_capacity(the_match.duration as usize);
    fn get_ratio(mat: &Match) -> f32 {
        let m = mat.game_state.field.m as f32;
        let n = mat.game_state.field.n as f32;
        let fc = mat.game_state.stats.filled_count as f32;
        fc / (m * n)
    }
    // random generator will supply seeds for bots
    let mut rng = the_match.random_seed
        .map(|seed| IsaacRng::new_from_u64(seed))
        .unwrap_or_else(|| IsaacRng::from_entropy());
    let mut random_seed_gen = move || rng.next_u64();
    for k in 0..nb {
        let idx = the_match.game_state.reordering[k] as usize;
        let seed: u64 = random_seed_gen();
        bots[idx].reset(&the_match.game_state, idx as u8, seed);
    }
    for tick in 0..the_match.duration {
        // if the cells has filled enough, do finish
        if get_ratio(the_match) >= the_match.ratio {
            break;
        }
        // now do loop for bots
        // tick + 1 because we want last iteration == allMoves.size
        the_match.game_state.stats.iteration = tick + 1;
        let mut moves = vec![Move::Stop; nb];
        // enumerate all the bots, move them
        for k in 0..nb {
            let idx = the_match.game_state.reordering[k] as usize;
            // let cgs = make_client_game_state(the_match.game_state, idx)
            let m = bots[idx].do_move(&the_match.game_state);
            step(&mut the_match.game_state, idx as u8, m);
            moves[idx] = m;
        }
        logger(&the_match.game_state);
        all_moves.push(moves);
    }
    Replay {
        height: the_match.game_state.field.m,
        width: the_match.game_state.field.n,
        duration: the_match.duration,
        ratio: the_match.ratio,
        player_names: the_match.game_state.player_names.clone(),
        moves: all_moves,
        random_seed: the_match.random_seed
    }
}

/// returns the final game state after the replay run
pub fn run_replay(replay: &Replay, logger: &Fn(&GameState)) -> GameState {
    let mut gs: GameState = create_match(
        replay.height,
        replay.width,
        &replay.player_names,
        replay.duration,
        replay.ratio,
        replay.random_seed
    ).game_state;
    for tick in 0..(replay.moves.len()) {
        gs.stats.iteration = (tick + 1) as u32;
        let np = gs.players.len();
        for k in 0..np {
            let idx = gs.reordering[k] as usize;
            let m = replay.moves[tick][idx];
            step(&mut gs, idx as u8, m);
            logger(&gs);
        }
    }
    gs
}
