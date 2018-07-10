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

fn main() {
    let random_seed: Option<u64> = Some(42);
    let np = 4;
    let perm0 = create_default_permutation(np);
    {
        let mut initializer_rng = random_seed.map(|seed| IsaacRng::new_from_u64(seed));
        let origin_perm = match &initializer_rng {
            Some(ref mut r) => copy_shuffled_permutation(&perm0, r),
            None => perm0.clone(),
        };
        let reordering = match &initializer_rng {
            Some(ref mut r) => copy_shuffled_permutation(&perm0, r),
            None => perm0.clone()
        };
        eprintln!("origin_perm = {:?}", origin_perm);
        eprintln!("reordering = {:?}", reordering);
    }
}

pub fn create_default_permutation(np: usize) -> Vec<u8> {
    (0..np).map(|x| x as u8).collect()
}

pub fn copy_shuffled_permutation(xs: &Vec<u8>, random: &mut RngCore) -> Vec<u8> {
    let mut tmp = xs.clone();
    random.shuffle(tmp.as_mut_slice());
    return tmp;
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
