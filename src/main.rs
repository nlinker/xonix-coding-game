#![allow(unused)]

extern crate rand;
extern crate byteorder;
extern crate xcg;
extern crate regex;
extern crate itertools;

use byteorder::{ByteOrder, LittleEndian};
use rand::prelude::{Rng, RngCore, SeedableRng, SmallRng, FromEntropy};
use rand::prng::XorShiftRng;
use rand::IsaacRng;
use std::mem::transmute_copy;
use xcg::utils::{Trim, IsaacRng0};
use xcg::model::GameState;
use regex::Regex;
use xcg::model::Point;
use std::fmt;
use itertools::Itertools;

use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {

    let gs: GameState = GameState::parse_string(&r#"
        *.*.*.*.*.*.*.
        *. . .1. . .*.
        *. a a a a A*.
        *. .1. . . .*.
        *.*.*.*.*.*.*B
    "#.trim_indent()).unwrap();
    eprintln!("pts = {}", gs);

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
