extern crate rand;

use rand::prelude::{RngCore, SeedableRng};

struct TestBot<'a> {
    path: &'a str,
    iter: u16,
    index: Option<u8>,
    random: Option<Box<RngCore + SeedableRng>>,
}


