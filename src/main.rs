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
use xcg::model::*;
use xcg::test::TestBot;
use xcg::utils::Trim;

fn main() {
    let mut buf = [0; 16];
    {
        let (mut b1, mut b2) = buf.split_at_mut(8);
        byteorder::LittleEndian::write_u64(&mut b1, 123);
        byteorder::LittleEndian::write_u64(&mut b2, 123);
    }
    let random = Rc::new(RefCell::new(XorShiftRng::from_seed(buf)));
    let random = Rc::new(RefCell::new(IsaacRng::new_from_u64(123)));

    let a = test_bot_r(0, random.clone(), "dllll");
    let b = test_bot_r(1, random.clone(), "luuuu");
    let c = test_bot_r(2, random.clone(), "urrrr");
    let d = test_bot_r(3, random.clone(), "rdddd");
    let mut bots = [a, b, c, d];
    let names = bots.iter().map(|bot| bot.name()).collect::<Vec<String>>();
    let logger = |gs: &GameState| {
        if gs.stats.iteration > 171 {
            println!("{}", prettify_game_state(gs, true, true));
            thread::sleep(Duration::from_millis(1));
        }
    };
    for _ in 0..100 {
        // run match
        let m = 20;
        let n = 20;
        let match_k_seed = (*random).borrow_mut().next_u64();
        let mut match_k = create_match(m, n, &names, 1024, 0.9, Some(match_k_seed));
        let replay_k = run_match(&mut match_k, &mut bots, &logger);
        println!("{}", "\n".repeat(m + names.len()))
    }
}

// reset to default color is \e[0m
// https://misc.flogisoft.com/bash/tip_colors_and_formatting
const COLORS: &'static [&'static str] = &[
    "\x1B[91m", "\x1B[92m", "\x1B[93m", "\x1B[94m",
    "\x1B[95m", "\x1B[96m", "\x1B[97m", "\x1B[90m",
    "\x1B[31m", "\x1B[32m", "\x1B[33m", "\x1B[34m",
    "\x1B[35m", "\x1B[36m", "\x1B[37m", "\x1B[30m",
];


pub fn prettify_game_state(gs: &GameState, rewind: bool, use_colors: bool) -> String {
    let m = gs.field.m;
    let n = gs.field.n;
    let np = gs.players.len();
    let capacity = if use_colors { m * (8 * n + 1) + 2 } else { m * 2 * (m + n) + 10 * np + 30 };
    let mut result = String::with_capacity(capacity);
    let mut layer0 = vec![vec![' ' as u8; n]; m];

    for i in 0..m {
        for j in 0..n {
            match gs.field.cells[i][j] {
                Cell::Empty => layer0[i][j] = '.' as u8,
                Cell::Border => layer0[i][j] = '*' as u8,
                Cell::Owned(k) => layer0[i][j] = ('0' as u8) + k,
            };
        }
    }
    for k in 0..np {
        let player = gs.players[k].body();
        for l in 0..player.len() {
            let i = player[l].0 as usize;
            let j = player[l].1 as usize;
            if l == player.len() - 1 {
                layer0[i][j] = ('A' as u8) + (k as u8);
            } else {
                layer0[i][j] = ('a' as u8) + (k as u8);
            }
        }
    }
    // now build the result string
    for k in 0..np {
        let mut s = String::with_capacity(n * 2);
        s.push_str(&format!("{}: {}", gs.player_names[k], gs.stats.scores[k]));
        let rest_len = n * 2 - 1 - s.len();
        for l in 0..rest_len {
            s.push(' ');
        }
        s.push('\n');
        result.push_str(&s);
    }
    for i in 0..m {
        for j in 0..n {
            if j == 0 {
                result.push(layer0[i][j] as char);
            } else {
                result.push(' ');
                result.push(layer0[i][j] as char);
            }
        }
        result.push('\n');
    }
    if rewind {
        for k in 0..(np + m + 1) {
            result.push_str("\x1B[A")
        }
    }
    result
}

fn test_bot_r<R: Rng>(idx: u8, rng: Rc<RefCell<R>>, path: &str) -> TestBot<R> {
    TestBot::with_index_random(path, idx, rng)
}

fn game_state(gs: &str) -> GameState {
    GameState::parse_string(&gs.trim_indent()).unwrap()
}
