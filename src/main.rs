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

//fn main() {
//    use xcg::model::*;
//    // test on the different cells sizes
//    let field_sizes = vec![(2, 2), (9, 3), (3, 4), (4, 7)];
//    //         n
//    //   * * * * * * *
//    // m *           *
//    //   *           *
//    //   * * * * * * *
//    for (m, n) in field_sizes {
//        let mut perimeter: Vec<Point> = vec![];
//        perimeter.append(&mut points_2(0, Box::new(0..n)));
//        perimeter.append(&mut points_1(Box::new(1..m - 1), n - 1));
//        perimeter.append(&mut points_2(m - 1, Box::new((0..n).rev())));
//        perimeter.append(&mut points_1(Box::new((1..m - 1).rev()), 0));
//        let size = 2 * (m + n - 2) as usize;
//        assert_eq!(size, perimeter.len());
//        for l in 0..=(2*size) {
//            assert_eq!(perimeter[l % size], border_to_point(m as usize, n as usize, l as usize));
//        }
//        eprintln!("perimeter = {:?}", perimeter);
//    }
//
//    fn points_1(ii: Box<Iterator<Item=i16>>, j: i16) -> Vec<Point>  {
//        return ii.map(|i| Point(i, j)).collect();
//    }
//
//    fn points_2(i: i16, jj: Box<Iterator<Item=i16>>) -> Vec<Point> {
//        return jj.map(|j| Point(i, j)).collect();
//    }
//}
