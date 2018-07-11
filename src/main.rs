// #![allow(unused)]

extern crate rand;
extern crate byteorder;
extern crate xcg;
extern crate regex;
extern crate itertools;

use rand::IsaacRng;
use std::fmt;
use std::cell::RefCell;
use rand::prelude::Rng;
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
struct TestBot<R: Rng> {
    path: Vec<u8>,
    iter: u32,
    random: Option<Rc<RefCell<R>>>,
}

impl<R: Rng> TestBot<R> {
    fn new(s: &str, rng: Rc<RefCell<R>>) -> TestBot<R> {
        let path = s.as_bytes().to_vec();
        let random = Some(rng);
        let iter = 0;
        TestBot { path, iter, random }
    }
}

impl<R: Rng + fmt::Debug> Bot for TestBot<R> {
    fn do_move(&mut self, _gs: &GameState) -> Move {
        if self.iter >= self.path.len() as u32 {
            let moves = vec![Move::Right, Move::Up, Move::Left, Move::Down];
            match self.random {
                None => Move::Stop,
                Some(ref mut r) => moves[r.borrow_mut().gen_range(0, moves.len())],
            }
        } else {
            let ch = self.path[self.iter as usize] as char;
            let m = match ch {
                'u' | 'U' => Move::Up,
                'd' | 'D' => Move::Down,
                'l' | 'L' => Move::Left,
                'r' | 'R' => Move::Right,
                's' | 'S' => Move::Stop,
                _ => panic!(format!("Invalid symbol: {}", ch))
            };
            self.iter += 1;
            m
        }
    }
}

fn main() {
    // shared across all the bots
    let teh_rng = Rc::new(RefCell::new(IsaacRng::new_from_u64(666)));

    let a = TestBot::new("d", teh_rng.clone());
    let b = TestBot::new("u", teh_rng.clone());
    let c = TestBot::new("d", teh_rng.clone());
    let d = TestBot::new("u", teh_rng.clone());
    let mut bots = [a, b, c, d];
    let mut gs = GameState { iteration: 0, data1: 10, data2: 20, data3: 30 };

    println!("round 1");
    round(&mut bots, &mut gs);
    println!("round 2");
    round(&mut bots, &mut gs);
    println!("round 3");
    round(&mut bots, &mut gs);
}

fn round<B: Bot>(bots: &mut [B], gs: &mut GameState) {
    for k in 0..bots.len() {
        let m = bots[k].do_move(&gs);
        println!("move: {:?} {:?}", k, m);
        step(gs, k as u8, m);
    }
}

fn step(gs: &mut GameState, _idx: u8, _mv: Move) {
    gs.data1 += 1;
    gs.data2 += 2;
    gs.data3 += 3;
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
