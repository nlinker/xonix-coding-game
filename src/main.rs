#![allow(unused)]

extern crate rand;
extern crate byteorder;
extern crate xcg;
extern crate regex;
extern crate itertools;

use byteorder::{ByteOrder, LittleEndian};
use rand::prelude::{RngCore, SeedableRng, SmallRng, FromEntropy};
use rand::prng::XorShiftRng;
use rand::IsaacRng;
use std::mem::transmute_copy;
use xcg::utils::{Trim, IsaacRng0};
use xcg::model::GameState;
use regex::Regex;
use xcg::model::Point;
use std::fmt;
use itertools::Itertools;


fn main() {
    let str0 = r#"
          *.*.*.*.*A*a*a
          *.3d2.2.2.0.*a
          *.2D2.2C2.1.*.
          *.2.2. . .1B*.
          *.*.*.*.*.*b*b
          reordering=[2,1,3,0]
          stats=Stats(19,33,2,1,0,[1,2,9,1])
          origins=[(0,6),(4,6),(4,0),(0,0)]
        "#.trim_indent();
    let gs = GameState::parse_string(&str0[..]).unwrap();
    let str1 = gs.to_string();
    assert_eq!(str0, str1);
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
