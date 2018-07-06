#![allow(unused)]

extern crate rand;
extern crate byteorder;
//extern crate xcg;
extern crate regex;
extern crate itertools;

use byteorder::{ByteOrder, LittleEndian};
use rand::prelude::{Rng, RngCore, SeedableRng, SmallRng, FromEntropy, ThreadRng};
use rand::prng::XorShiftRng;
use rand::IsaacRng;
use std::mem::transmute_copy;
//use xcg::utils::{Trim, IsaacRng0};
//use xcg::model::GameState;
//use xcg::model::step;
//use xcg::model::Move;
//use xcg::model::Point;
use regex::Regex;
use std::fmt;
use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, PartialOrd)]
pub struct Point(pub i16, pub i16);

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Field {
    pub m: usize,
    pub n: usize,
    pub cells: Vec<Vec<u8>>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Stats {
    pub iteration: u16,
    pub scores: Vec<u16>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct GameState {
    pub field: Field,
    pub stats: Stats,
    pub players: Vec<Vec<Point>>,
}

pub enum Move {
    Right, Up, Left, Down, Stop,
}

fn main() {
    let cells: Vec<Vec<u8>> = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];
    let field = Field { cells, m: 5, n: 5 };
    let stats = Stats { iteration: 0, scores: vec![0, 0] };
    let players = vec![vec![Point(0, 0)], vec![Point(4, 4)]];
    let mut gs = GameState { field, stats, players };
    game_step(&mut gs, 0, Move::Up);
    eprintln!("gs changed = {:?}", gs);
}


fn game_step(gs: &mut GameState, idx: usize, mv: Move) {
    let mut rng = rand::thread_rng();
    let np = gs.players.len();
    let m = gs.field.m as i16;
    let n = gs.field.n as i16;

    let old_head = gs.players[idx][0];
    let new_head = calculate_head(&gs.field, old_head, mv);
    // let mut stats = &gs.stats;

    // the player hasn't effectively moved
    if old_head == new_head {
        return;
    }

    let collision = (0..np)
        .filter(|k| gs.players[*k].contains(&new_head))
        .map(|k| (k as u8, &gs.players[k]))
        .collect::<Vec<(u8, &Vec<Point>)>>();

    if new_head != old_head && !collision.is_empty() {
        if rng.gen() {
            gs.players[idx].push(Point(0, 1));
            gs.stats.iteration += 1;
            gs.stats.scores[0] += 1;
        } else {
            gs.players[idx].clear();
            gs.players[idx].push(Point(m - 1, n - 1));
            gs.stats.iteration += 1;
            gs.stats.scores[1] += 2;
        }
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

fn has_inside(field: &Field, p: Point) -> bool {
    let Point(i, j) = p;
    0 <= i && i < (field.m as i16) && 0 <= j && j < (field.n as i16)
}

//fn main() {
////    let mut rng = IsaacRng::new_from_u64(123456u64);
//    let mut rng = IsaacRng::from_entropy();
//    let rng0: IsaacRng0 = unsafe { transmute_copy(&rng) };
//    println!("{:?}", rng0);
//
//    let mut results = [0u32; 20];
//    for i in results.iter_mut() {
//        *i = rng.next_u32();
//    }
//    println!("{:?}", results);
//    println!("rng: {:?}", rng);
//}
