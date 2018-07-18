#![allow(unused)]

extern crate core;
extern crate rand;
extern crate byteorder;
extern crate xcg;
extern crate regex;
extern crate itertools;

use byteorder::{ByteOrder, LittleEndian as LE};
use rand::{IsaacRng, XorShiftRng};
use rand::prelude::{Rng, RngCore, SeedableRng};
use std::fmt;
use std::cell::RefCell;
use std::time::Duration;
use std::thread;
use std::rc::Rc;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, PartialOrd)]
pub struct Point(pub i16, pub i16);

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Move {
    Right, Up, Left, Down, Stop,
}

struct GameState {
    iteration: u16,
}

trait Bot {
    fn do_move(&mut self, gs: &GameState) -> Move;
}

struct Bot1 {
    random: RefCell<IsaacRng>,
    path: Vec<Point>,
    path_pos: Option<usize>,
}

impl Bot1 {
    fn new(seed: u64) -> Self {
        Bot1 {
            random: RefCell::new(IsaacRng::new_from_u64(seed)),
            path: vec![],
            path_pos: None,
        }
    }
}

fn main() {
    let mut b = Bot1::new(12);
    b.path = vec![Point(0, 0), Point(0, 1), Point(0, 2)];
    b.path_pos = Some(1);
}

//use xcg::model::*;
//use xcg::bot::TestBot;
//use xcg::bot::Bot1;
//use xcg::bot::Bot2;
//use xcg::utils::Trim;
//
//fn main() {
////    let mut buf = [0; 16];
////    {
////        let (mut b1, mut b2) = buf.split_at_mut(8);
////        byteorder::LittleEndian::write_u64(&mut b1, 123);
////        byteorder::LittleEndian::write_u64(&mut b2, 123);
////    }
////    let random = Rc::new(RefCell::new(XorShiftRng::from_seed(buf)));
//    let random = Rc::new(RefCell::new(IsaacRng::new_from_u64(123)));
//    let m = 24;
//    let n = 30;
//
//    let a = Bot1::new(0);
//    let b = Bot1::new(1);
//    let c = Bot2::new(2);
//    let d = Bot2::new(3);
//    //let mut bots = [a];
//    let mut bots: [Box<Bot>; 4] = [Box::new(a), Box::new(b), Box::new(c), Box::new(d)];
//    let names: Vec<String> = bots.iter().enumerate()
//        .map(|(k, _)| ((('A' as u8) + (k as u8)) as char).to_string())
//        .collect();
//
//    let logger = |gs: &GameState| {
//        if gs.stats.iteration > 0 {
//            println!("{}", prettify_game_state(gs, true, true));
//            thread::sleep(Duration::from_millis(10));
//        }
//    };
//    for _ in 0..100 {
//        // run match
//        let match_k_seed = (*random).borrow_mut().next_u64();
//        let mut match_k = create_match(m, n, &names, 1024, 0.9, Some(match_k_seed));
//        let replay_k = run_match(&mut match_k, &mut bots, &logger);
//        println!("{} {:?}", "\n".repeat(m + names.len()), match_k.game_state.stats);
//    }
//}
//
//fn test_bot_r<R: Rng>(idx: u8, rng: Rc<RefCell<R>>, path: &str) -> TestBot<R> {
//    TestBot::with_index_random(path, idx, rng)
//}
//
//fn game_state(gs: &str) -> GameState {
//    GameState::parse_string(&gs.trim_indent()).unwrap()
//}
