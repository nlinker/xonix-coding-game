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
use std::borrow::Borrow;
use std::borrow::BorrowMut;

#[derive(Debug)]
struct Match{
    data: u32,
    random_seed: Option<u64>,
}

struct GameState {
    iteration: u16,
}

trait Bot { fn do_this(&mut self, state: GameState); }

#[derive(Debug)]
struct TestBot<R: Rng> {
    random: Option<R>,
    some_data: u32,
}

impl<R: Rng> TestBot<R> {
    fn new(ss: &str) -> TestBot<R> {
        TestBot {
            random: None,
            some_data: ss.len() as u32
        }
    }
}

impl<R: Rng + fmt::Debug> Bot for TestBot<R> {
    fn do_this(&mut self, gs: GameState) {
        if let Some(ref mut r) = self.random {
            self.some_data = r.gen();
        }
        eprintln!("&self = {:#?}", &self);
    }
}


fn main() {
    let random_seed = None;
    let mut initializer_rng = random_seed.map(|seed| IsaacRng::new_from_u64(seed));
    let a = TestBot::<IsaacRng>::new("dlu");
    let b = TestBot::<IsaacRng>::new("llurr");
    let c = TestBot::<IsaacRng>::new("urd");
    let d = TestBot::<IsaacRng>::new("rrrdlll");
    let bots = vec![a, b, c, d];
    let m1 = create_match(&bots, None);
    eprintln!("m1 = {:#?}", m1);
}

fn create_match<'a, B: Bot + 'a>(bots: &Vec<B>, seed: Option<u64>) -> Match {
    let rng = seed.map(|s| IsaacRng::new_from_u64(s));
    Match {
        data: rng.map(|ref mut r| r.gen()).unwrap_or(0),
        random_seed: seed,
    }
}


fn create_default_permutation(np: usize) -> Vec<u8> {
    (0..np).map(|x| x as u8).collect()
}

fn copy_shuffled_permutation(xs: &Vec<u8>, random: &mut RngCore) -> Vec<u8> {
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
