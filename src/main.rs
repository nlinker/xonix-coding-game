#![allow(unused)]

extern crate rand;
extern crate byteorder;
extern crate xcg;
extern crate regex;
extern crate itertools;

use byteorder::{ByteOrder, LittleEndian};
use rand::prelude::{Rng, RngCore, SeedableRng, SmallRng, FromEntropy, ThreadRng};
use rand::prng::XorShiftRng;
use rand::IsaacRng;
use regex::Regex;
use std::mem::transmute_copy;
use std::fmt;
use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Move {
    Right, Up, Left, Down, Stop,
}

trait Bot {
    // the bot is mutable
    fn reset(&mut self, idx: u8, gs: &GameState);
    fn do_move(&mut self, idx: u8, gs: &GameState) -> Move;
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Stats {
    pub iteration: u16,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct GameState {
    pub stats: Stats,
    pub reordering: Vec<u8>,
}

impl fmt::Display for GameState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("GameState")
    }
}

////////////////// Bot
struct TestBot<R: Rng> {
    path: Vec<u8>,
    iter: u32,
    random: Option<R>,
}

impl<R: Rng> TestBot<R> {
    pub fn new(s: &str) -> TestBot<R> {
        let path = s.as_bytes().to_vec();
        TestBot { path, iter: 0, random: None }
    }
}

impl<R: Rng> Bot for TestBot<R> {
    fn reset(&mut self, _idx: u8, _gs: &GameState) {}
    fn do_move(&mut self, _idx: u8, _gs: &GameState) -> Move { Move::Stop }
}
////////////////// Bot end

fn main() {
    let reordering = vec![0, 1];
    let gs0 = GameState { stats: Stats { iteration: 0 }, reordering };
    let a = test_bot("u");
    let gs1 = play(&gs0, &mut [a]);
    eprintln!("gs1 = {}", &gs1);
}

fn test_bot(path: &str) -> TestBot<IsaacRng> {
    TestBot::new(path)
}

fn play<B: Bot>(gs: &GameState, bots: &mut [B]) -> GameState {
    let mut gs = gs.clone();
    let mut progressing = true;
    let mut iteration = 0;
    while progressing {
        iteration += 1;
        gs.stats.iteration = iteration;
        let mut moves = vec![];
        for k in 0..bots.len() {
            let idx = gs.reordering[k];
            // let cgs = game.make_client_game_state(gs, idx);
            let mv = bots[idx as usize].do_move(idx, &gs);
            moves.push(mv);
            step(&mut gs, idx, mv);
        }
        if moves.iter().all(|m| *m == Move::Stop) {
            progressing = false;
        }
    }
    gs
}

fn step(gs: &mut GameState, idx: u8, mv: Move) {

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
