extern crate rand;

use rand::prelude::Rng;
use model::Move;
use model::Point;
use std::cell::RefCell;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Bot2<R: Rng> {
    index: u8,
    m: usize,
    n: usize,
    random: RefCell<R>,
    destination: Option<Point>,
    last_move: Option<Move>,
}
