#![allow(unused)]

extern crate rand;
extern crate byteorder;
extern crate xcg;
extern crate regex;
extern crate itertools;

use byteorder::{ByteOrder, LittleEndian};
use rand::prelude::{Rng, RngCore, SeedableRng, SmallRng, FromEntropy, ThreadRng};
use rand::prng::XorShiftRng;
use rand::IsaacRng;
use regex::Regex;
use std::mem::transmute_copy;
use std::fmt;
use itertools::Itertools;

use xcg::utils::{Trim, IsaacRng0};
use xcg::model::*;
use xcg::test::TestBot;


fn main() {
    let gs0 = game_state(r#"
            *.*.*.*.*.*.*.
            *. b B . A .*.
            *. a a a a .*.
            *. . . . . .*.
            *.*.*.*.*.*.*.
        "#);
    let a = test_bot("u");
    let gs1 = play(&gs0, &mut [a]);
    eprintln!("gs1 = {}", &gs1);
}

fn game_state(gs: &str) -> GameState {
    GameState::parse_string(&gs.trim_indent()).unwrap()
}

fn test_bot(path: &str) -> TestBot<IsaacRng> {
    TestBot::new(path)
}

fn play<B: Bot>(gs: &GameState, bots: &mut [B]) -> GameState {
    let mut gs = gs.clone();
    let mut progressing = true;
    let mut iteration = 0;
    while progressing {
        iteration += 1;
        gs.stats.iteration = iteration;
        let mut moves = vec![];
        for k in 0..bots.len() {
            let idx = gs.reordering[k];
            // let cgs = game.make_client_game_state(gs, idx);
            let mv = bots[idx as usize].do_move(idx, &gs);
            moves.push(mv);
            step(&mut gs, idx, mv);
        }
        if moves.iter().all(|m| *m == Move::Stop) {
            progressing = false;
        }
    }
    gs
}

//fn main() {
////    let mut rng = IsaacRng::new_from_u64(123456u64);
//    let mut rng = IsaacRng::from_entropy();
//    let rng0: IsaacRng0 = unsafe { transmute_copy(&rng) };
//    println!("{:?}", rng0);
//
//    let mut results = [0u32; 20];
//    for i in results.iter_mut() {
//        *i = rng.next_u32();
//    }
//    println!("{:?}", results);
//    println!("rng: {:?}", rng);
//}
