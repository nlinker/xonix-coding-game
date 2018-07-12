extern crate rand;

use rand::prelude::Rng;
use model::Bot;
use model::GameState;
use model::Move;
use std::cell::RefCell;
use std::rc::Rc;

// TODO move it to ../tests/test_bot.rs
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TestBot<R: Rng> {
    path: Vec<u8>,
    iter: u32,
    idx: Option<u8>,
    random: Option<Rc<RefCell<R>>>,
}

impl<R: Rng> TestBot<R> {
    pub fn new(s: &str) -> TestBot<R> {
        let path = s.as_bytes().to_vec();
        TestBot { path, iter: 0, idx: None, random: None }
    }
    pub fn with_index_random(s: &str, idx: u8, rng: Rc<RefCell<R>>) -> TestBot<R> {
        let path = s.as_bytes().to_vec();
        TestBot { path, iter: 0, idx: Some(idx), random: Some(rng) }
    }
    pub fn name(&self) -> String {
//        self.idx.map(|id| {
//            let ch = (('A' as u8) + id) as char;
//            format!("{}: {}", ch, from_utf8(&self.path).unwrap())
//        }).unwrap_or_else(|| format!("_: {}", from_utf8(&self.path).unwrap()))
        self.idx.map(|id| format!("{}", ((('A' as u8) + id) as char).to_string()))
            .unwrap_or_else(|| "?".to_string())
    }
}

impl<R: Rng> Bot for TestBot<R> {

    fn reset(&mut self, _gs: &GameState, idx: u8, seed: u64) {
        // reset the inner state
        println!("reset state index={} seed={}", idx, seed)
    }

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

