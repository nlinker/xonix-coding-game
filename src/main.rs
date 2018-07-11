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
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct GameState {
    iteration: u16,
    data1: u32,
    data2: u32,
    data3: u32,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Move {
    Right, Up, Left, Down, Stop,
}

trait Bot {
    fn do_move(&mut self, gs: &GameState) -> Move;
}

#[derive(Debug)]
struct TestBot<'a, R: Rng + 'a> {
    path: Vec<u8>,
    iter: u32,
    random: Option<&'a RefCell<R>>,
}

impl<'a, R: Rng> TestBot<'a, R> {
    fn new(s: &str, rng: &'a RefCell<R>) -> TestBot<'a, R> {
        let path = s.as_bytes().to_vec();
        let random = Some(rng);
        let iter = 0;
        TestBot { path, iter, random }
    }
}

impl<'a, R: Rng + fmt::Debug> Bot for TestBot<'a, R> {
    fn do_move(&mut self, gs: &GameState) -> Move {
        Move::Stop
    }
}

fn main() {
    // shared across all the bots
    let teh_rng = RefCell::new(IsaacRng::new_from_u64(666));

    let a = TestBot::<IsaacRng>::new("ddd", &teh_rng);
    let b = TestBot::<IsaacRng>::new("uuu", &teh_rng);
    let c = TestBot::<IsaacRng>::new("ddd", &teh_rng);
    let d = TestBot::<IsaacRng>::new("uuu", &teh_rng);
    let mut bots = [a, b, c, d];
    let gs = GameState { iteration: 0, data1: 10, data2: 20, data3: 30 };

    println!("round 1");
    for k in 0..bots.len() {
        let m = bots[k].do_move(&gs);
        eprintln!("moves: {:?} {:?}", k, m);
    }
    println!("round 2");
    for k in 0..bots.len() {
        let m = bots[k].do_move(&gs);
        eprintln!("moves: {:?} {:?}", k, m);
    }
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
