extern crate rand;

use rand::prelude::Rng;
use model::Bot;
use model::GameState;
use model::Move;
use std::str::from_utf8;
use std::cell::RefCell;

// TODO move it to ../tests/test_bot.rs
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TestBot<R: Rng> {
    path: Vec<u8>,
    iter: u32,
    idx: Option<u8>,
    rng: Option<RefCell<R>>,
}

impl<R: Rng> TestBot<R> {
    pub fn new(s: &str) -> TestBot<R> {
        let path = s.as_bytes().to_vec();
        TestBot { path, iter: 0, idx: None, rng: None }
    }
    pub fn with_random(s: &str, idx: u8, _rng: &R) -> TestBot<R> {
        let path = s.as_bytes().to_vec();
        TestBot { path, iter: 0, idx: Some(idx), rng: None
            // TODO rng: Some(rng)
        }
    }
    pub fn name(&self) -> String {
        self.idx.map(|id| {
            let ch = (('A' as u8) + id) as char;
            format!("{}: {}", ch, from_utf8(&self.path).unwrap())
        }).unwrap_or_else(|| format!("_: {}", from_utf8(&self.path).unwrap()))
    }
}

impl<R: Rng> Bot for TestBot<R> {

    fn reset(&mut self, _gs: &GameState, _idx: u8, _seed: u64) {
        // reset the inner state
    }

    fn do_move(&mut self, _gs: &GameState) -> Move {
        if self.iter >= self.path.len() as u32 {
            let moves = vec![Move::Right, Move::Up, Move::Left, Move::Down];
            match self.rng.as_mut() {
                None => Move::Stop,
                Some(r) => moves[r.borrow_mut().gen_range(0, moves.len())],
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

