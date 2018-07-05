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

use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::VecDeque;
//#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
//struct Point(pub i16, pub i16);

fn main() {
    let result: RefCell<HashSet<Point>> = RefCell::new(HashSet::new());

    let cells: Vec<Vec<u8>> = vec![
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];
    let pts = flood(&cells, 5, 5, Point(0, 0));
    eprintln!("pts = {:?}", pts);

}

fn flood(cells: &Vec<Vec<u8>>, m: i16, n: i16, start: Point) -> HashSet<Point> {
    let neighbors = vec![Point(0, -1), Point(-1, 0), Point(0, 1), Point(1, 0)];

    // result is the growing set of points describing the filled area
    let result: RefCell<HashSet<Point>> = RefCell::new(HashSet::new());

    // local function 1
    let has_inside: Box<Fn(Point) -> bool> = Box::new(|Point(i, j)| {
        0 <= i && i < m && 0 <= j && j < n
    });

    // local function 2
    let in_area: Box<Fn(Point)->bool> = Box::new(|p| {
        has_inside(p) && !result.borrow().contains(&p) && cells[p.0 as usize][p.1 as usize] == 0
    });

    if !in_area(start) {
        // if the starting point on the boundary
        return result.clone().into_inner();
    }

    // starting point is somewhere inside
    let mut queue: VecDeque<Point> = VecDeque::with_capacity((m + n) as usize);
    queue.push_back(start);
    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();

        // result is changed here!
        result.borrow_mut().insert(cur);

        let mut candidates = neighbors.iter()
            .map(|p| Point(cur.0 + p.0, cur.1 + p.1))
            .filter(|p| in_area(*p) && !queue.contains(p))
            .collect();
        queue.append(&mut candidates);
    }
    result.clone().into_inner()
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
