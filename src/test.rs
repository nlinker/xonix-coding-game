extern crate rand;

use rand::prelude::Rng;
use model::Bot;
use model::GameState;
use model::Move;

// TODO move it to ../tests/test_bot.rs

pub struct TestBot<R: Rng> {
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

    fn do_move(&mut self, _idx: u8, _gs: &GameState) -> Move {
        if self.iter >= self.path.len() as u32 {
            let moves = vec![Move::Right, Move::Up, Move::Left, Move::Down];
            match self.random.as_mut() {
                None => Move::Stop,
                Some(r) => moves[r.gen_range(0, moves.len())],
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

