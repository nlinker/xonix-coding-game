#![allow(unused)]

pub mod utils;

extern crate rand;
extern crate byteorder;
extern crate xcg;

use rand::prelude::{RngCore, SeedableRng, SmallRng, FromEntropy};
use rand::prng::XorShiftRng;
use rand::IsaacRng;
use byteorder::{ByteOrder, LittleEndian};
use std::mem::transmute_copy;
use utils::IsaacRng0;

fn main() {
//    let mut rng = IsaacRng::new_from_u64(123456u64);
    let mut rng = IsaacRng::from_entropy();
    let rng0: IsaacRng0 = unsafe { transmute_copy(&rng) };
    println!("{:?}", rng0);

    let mut results = [0u32; 20];
    for i in results.iter_mut() {
        *i = rng.next_u32();
    }
    println!("{:?}", results);
    println!("rng: {:?}", rng);
}
