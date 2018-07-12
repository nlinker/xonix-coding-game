#![allow(unused)]

extern crate rand;
extern crate byteorder;
extern crate xcg;
extern crate regex;
extern crate itertools;


use byteorder::{ByteOrder, LittleEndian as LE};
use rand::IsaacRng;
use std::fmt;
use std::cell::RefCell;
use rand::prelude::{Rng, RngCore, SeedableRng};
use std::rc::Rc;
use xcg::model::*;
use xcg::test::TestBot;
use xcg::utils::Trim;
use rand::XorShiftRng;

fn main() {
    let mut buf = [0; 16];
    {
        let (mut b1, mut b2) = buf.split_at_mut(8);
        byteorder::LittleEndian::write_u64(&mut b1, 123);
        byteorder::LittleEndian::write_u64(&mut b2, 123);
    }
//    let random = Rc::new(RefCell::new(XorShiftRng::from_seed(buf)));
    let random = Rc::new(RefCell::new(IsaacRng::new_from_u64(123)));

    let a = test_bot_r(0, random.clone(), "dllll");
    let b = test_bot_r(1, random.clone(), "luuuu");
    let c = test_bot_r(2, random.clone(), "urrrr");
    let d = test_bot_r(3, random.clone(), "rdddd");
    let mut bots = [a, b, c, d];
    let names = bots.iter().map(|bot| bot.name()).collect::<Vec<String>>();
    let logger = |_gs: &GameState| {};
    for _ in 0..100 {
        // run match
        let match_k_seed = (*random).borrow_mut().next_u64();
        let mut match_k = create_match(11, 11, &names, 32, 0.9, Some(match_k_seed));
        let replay_k = run_match(&mut match_k, &mut bots, &logger);
        let gs1 = run_replay(&replay_k, &logger);
        let gs2 = run_replay(&replay_k, &logger);
        debug_assert_eq!(match_k.game_state, gs1);
        debug_assert_eq!(match_k.game_state, gs2);
    }

}

fn test_bot_r<R: Rng>(idx: u8, rng: Rc<RefCell<R>>, path: &str) -> TestBot<R> {
    TestBot::with_index_random(path, idx, rng)
}

fn game_state(gs: &str) -> GameState {
    GameState::parse_string(&gs.trim_indent()).unwrap()
}
