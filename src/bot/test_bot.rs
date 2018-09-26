use crate::model::Bot;
use crate::model::Move;
use crate::model::GameStateView;
use std::cell::RefCell;
use std::rc::Rc;
use rand::prelude::Rng;

// TODO move it to ../tests/test_bot.rs
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TestBot<R: Rng> {
    path: Vec<u8>,
    iter: u32,
    idx: Option<usize>,
    random: Option<Rc<RefCell<R>>>,
}

impl<R: Rng> TestBot<R> {
    pub fn new(s: &str) -> TestBot<R> {
        let path = s.as_bytes().to_vec();
        TestBot { path, iter: 0, idx: None, random: None }
    }
    pub fn with_index_random(s: &str, idx: usize, rng: Rc<RefCell<R>>) -> TestBot<R> {
        let path = s.as_bytes().to_vec();
        TestBot { path, iter: 0, idx: Some(idx), random: Some(rng) }
    }
}

impl<R: Rng> Bot for TestBot<R> {

    fn reset(&mut self, _gs: &GameStateView, idx: usize, _seed: u64) {
        self.iter = 0;
        self.idx = Some(idx);
        // reset the inner state
        // println!("reset state index={} seed={}", idx, seed)
    }

    fn do_move(&mut self, _gs: &GameStateView) -> Move {
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
