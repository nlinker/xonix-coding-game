extern crate rand;
extern crate xcg;
extern crate console;
extern crate crossbeam;

use rand::IsaacRng;
use rand::prelude::RngCore;
use std::cell::RefCell;
use console::style;

use std::thread;
use std::time::Duration;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Cell {
    Empty, Border, Owned(u8)
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, PartialOrd)]
pub struct P(pub i16, pub i16);

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Weight { f: i32, g: i32 }

// global instance
static WEIGHT: &'static HashMap<P, Weight> = HashMap::new();

impl Ord for P {
    fn cmp<'a>(&self, other: &'a Self) -> Ordering {
        let w1 = WEIGHT.get(self);
        let w2 = WEIGHT.get(other);
        if w1.is_some() && w2.is_some() {
            w1.unwrap().f.cmp(&w2.unwrap().f)
        } else {
            Ordering::Equal
        }
    }
}

fn main() {
    let random = RefCell::new(IsaacRng::new_from_u64(234));
    let m = 8;
    let n = 10;
    let mut cells = vec![vec![Cell::Empty; m]; n];
}
