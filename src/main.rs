#![allow(unused)]

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
use xcg::model::*;
use xcg::test::TestBot;
use xcg::utils::Trim;

fn main() {

}

fn test_bot_r<R: Rng>(idx: u8, rng: Rc<RefCell<R>>, path: &str) -> TestBot<R> {
    TestBot::with_index_random(path, idx, rng)
}

fn game_state(gs: &str) -> GameState {
    GameState::parse_string(&gs.trim_indent()).unwrap()
}
