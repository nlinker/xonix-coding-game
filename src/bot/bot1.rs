extern crate rand;

use rand::prelude::Rng;
use rand::IsaacRng;
use model::Move;
use std::cell::RefCell;
use model::Point;


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Bot1<R: Rng> {
    index: u8,
    m: usize,
    n: usize,
    random: RefCell<R>,
    destination: Option<Point>,
    last_move: Option<Move>,
}

//impl Bot1 {
//    pub fn new(index: u8) -> Self {
//        Bot1 { index, m: 0, n: 0, random: None, destination: None, last_move: None }
//    }
//}
//
//impl Bot for Bot1 {
//    fn reset(&mut self, gs: &GameState, idx: u8, seed: u64) {
//        self.m = gs.field.m;
//        self.n = gs.field.n;
//        self.index = idx;
//        self.random = R::new;
//    }
//
//    fn do_move(&mut self, gs: &GameState) -> Move {
//        Move::Stop
//    }
//}
//
//fn direction(src: Point, p: Point) -> Move {
//    let si = src.0;
//    let sj = src.1;
//    let di = p.0;
//    let dj = p.1;
//    if di == si && dj <= sj {
//        Move::Left
//    } else if di == si && dj > sj {
//        Move::Right
//    } else if di < si {
//        UP
//    } else {
//        DOWN
//    }
//}
