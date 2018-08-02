#![allow(unused)]

extern crate rand;
extern crate xcg;
extern crate console;
extern crate crossbeam;
extern crate priority_queue;

use rand::IsaacRng;
use rand::prelude::RngCore;
use std::cell::RefCell;

use xcg::model::*;
use xcg::bot::Bot2;
use std::thread;
use std::time::Duration;

use xcg::utils::Trim;
use xcg::model::*;
use xcg::model::prettify_game_state;
use xcg::bot::common::{P, a_star_find};
use xcg::bot::common::distance;
use xcg::bot::common::Weight;
use priority_queue::PriorityQueue;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    let random = RefCell::new(IsaacRng::new_from_u64(234));
    let m = 32;
    let n = 54;
//    let m = 32;
//    let n = 54;
    let timeout = 40;

    let a = Bot2::new(0);
    let b = Bot2::new(1);
    let c = Bot2::new(2);
    let d = Bot2::new(3);
//    let mut bots: [Box<dyn Bot>; 2] = [Box::new(a), Box::new(b)];
    let mut bots: [Box<dyn Bot>; 4] = [Box::new(a), Box::new(b), Box::new(c), Box::new(d)];
    let names: Vec<String> = bots.iter().enumerate()
        .map(|(k, _)| ((('A' as u8) + (k as u8)) as char).to_string())
        .collect();

    let logger = |gs: &GameState| {
        if gs.stats.iteration > 0 {
            println!("{}", prettify_game_state(gs, true, true));
            thread::sleep(Duration::from_millis(timeout));
        }
    };

    let count = 100_000;
    let random = RefCell::new(IsaacRng::new_from_u64(234));
    let mut seeds = Vec::with_capacity(count);
    for it in 0..count {
        let match_k_seed = random.borrow_mut().next_u64();
        seeds.push(match_k_seed);
    }

    for it in 0..1 {
//        let seed = random.borrow_mut().next_u64();
        let seed = 10591930711989851205;
//        let seed = seeds[it];
        let mut match_k = create_match(m, n, &names, 1024, 0.95, Some(seed));
        let _replay_k = run_match(&mut match_k, &mut bots, &logger);
//        println!("{} {:?}", "\n".repeat(m + names.len()), match_k.game_state.stats);
        let stats = match_k.game_state.stats.clone();
        let i = stats.iteration;
        let o = stats.ouroboros_count;
        let b = stats.bite_count;
        let h = stats.head_to_head_count;
        let s = stats.scores;
        println!("{:06}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", it, i, o, b, h, seed, s[0], s[1], s[2], s[3]);
//        println!("{:06}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", it, i, o, b, h, seed, s[0], s[1]);
        println!("{}", prettify_game_state(&match_k.game_state, false, true));
    }
}
