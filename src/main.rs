#![allow(unused)]

extern crate rand;
extern crate byteorder;
extern crate xcg;

use byteorder::{ByteOrder, LittleEndian};
use rand::prelude::{RngCore, SeedableRng, SmallRng, FromEntropy};
use rand::prng::XorShiftRng;
use rand::IsaacRng;
use std::mem::transmute_copy;
use xcg::utils::{Trim, IsaacRng0};
use xcg::model::GameState;

fn main() {
    let str0 = r#"
      reordering=[2,1,3,0]
      stats=Stats(19,33,2,1,0,[1,2,9,1])
      origins=[(0,6),(4,6),(4,0),(0,0)]
    "#.trim_indent();
    let rest: Vec<&str> = str0
        .split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    let prr = GameState::parse_string_rest(4, &rest);
    eprintln!("rest = {:?}", prr);
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
