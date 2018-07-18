#![allow(unused)]

use model::Bot;
use model::Cell;
use model::GameState;
use model::Move;
use model::Point;
use rand::IsaacRng;
use rand::prelude::{Rng, FromEntropy};
use std::cell::RefCell;
use utils::Bound;

#[derive(Clone, Debug)]
pub struct Bot2 {
    idx: usize,
    random: RefCell<IsaacRng>,
    m: usize,
    n: usize,
    cur_me: Vec<Point>,
    last_me: Vec<Point>,
    path: Vec<Point>,
    path_pos: usize,
    gs: Option<Box<GameState>>,
}

impl Bot for Bot2 {
    fn reset(&mut self, gs: &GameState, idx: u8, seed: u64) {
        self.idx = idx as usize;
        self.m = gs.field.m;
        self.n = gs.field.n;
        self.random = RefCell::new(IsaacRng::new_from_u64(seed));
    }

    fn do_move(&mut self, gs: &GameState) -> Move {
        self.last_me = self.cur_me.clone();
        self.cur_me = gs.players[self.idx].body().iter().map(|p| self.to_decartes(p)).collect();
        if self.cur_me.is_empty() {
            return Move::Stop;
        }
        let cur_head = gs.players[self.idx].head().unwrap();
        // if we were flooded or bitten, then reset the path
        if self.cur_me.len() < self.last_me.len() {
            self.path = vec![];
            self.path_pos = 0;
        }

        let the_move = if !self.path.is_empty() {
            if self.path_pos < self.path.len() {
                self.path_pos += 1;
            } else {
                self.path_pos = 0;
            }
            let new_head = self.path.first().unwrap();
            direction(cur_head, new_head)
        } else {
            // generate the new path
            let mut empties = self.find_random_empty(20);
            &empties.sort_by_key(|p| distance(cur_head, p));
            // now try to take approximately 5th element
            let the_empty = &empties[..5].last();
            eprintln!("the_empty = {:#?}", the_empty);
            Move::Stop
        };

        the_move
    }
}

impl Bot2 {
    pub fn new(idx: u8) -> Self {
        Bot2 {
            idx: idx as usize,
            random: RefCell::new(IsaacRng::from_entropy()),
            m: 0,
            n: 0,
            cur_me: vec![],
            last_me: vec![],
            path: vec![],
            path_pos: 0,
            gs: None
        }
    }

    /// Note: Decartes coordinates to accept
    fn cells(&self, p: &Point) -> Cell {
        let gs = &self.gs.unwrap();
        let from_decartes_x = |x: i16| x as usize;
        let from_decartes_y = |y: i16| gs.field.m - 1 - (y as usize);
        gs.field.cells[from_decartes_y(p.1)][from_decartes_x(p.0)]
    }

    fn to_decartes(&self, p: &Point) -> Point {
        Point(p.1, (self.m as i16) - 1 - p.0)
    }

    fn find_random(&self, attempts: usize, predicate: impl Fn(&Point) -> bool) -> Vec<Point> {
        let mut buf: Vec<Point> = Vec::with_capacity(attempts);
        for _ in 0..attempts {
            let x = self.random.borrow_mut().gen_range(0, self.n as i16);
            let y = self.random.borrow_mut().gen_range(0, self.m as i16);
            let p = Point(x, y);
            if predicate(&p) {
                buf.push(p)
            }
        }
        buf
    }

    fn find_random_empty(&self, attempts: usize) -> Vec<Point> {
        self.find_random(attempts, |p| self.cells(&p) == Cell::Empty)
    }

}


fn distance(p: &Point, q: &Point) -> i16 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn may_be_selected(o: Point, a: Point, x: Point) -> bool {
    let Point(oi, oj) = o;
    let Point(ai, aj) = a;
    let Point(xi, xj) = x;
    // 4 3 2
    // 5 9 1
    // 6 7 8
    if false { false }
    else if oi == ai && oj < aj { aj <= xj }
    else if oi > ai && oj < aj  { xi <= ai && aj <= xj }
    else if oi > ai && oj == aj { xi <= ai }
    else if oi > ai && oj > aj  { xi <= ai && xj <= aj }
    else if oi == ai && oj > aj { xj <= aj }
    else if oi < ai && oj > aj  { ai <= xi && xj <= aj }
    else if oi < ai && oj == aj { ai <= xi }
    else if oi < ai && oj < aj  { ai <= xi && aj <= xj }
    else if oi == ai && oj < aj { aj <= xj }
    else                        { ai != xi && aj != xj }
}

fn border_or_owned_partial(gs: &GameState, o: Point, a: Point, p: Point) -> bool {
    let cell = gs.field.cells[p.0 as usize][p.1 as usize];
    let selected = may_be_selected(o, a, p);
    (cell != Cell::Empty) && selected
}

fn find_closest(gs: &GameState, src: Point, predicate: impl Fn(&Point) -> bool) -> Option<Point> {
    let Point(oi, oj) = src;
    let m = gs.field.m as i16;
    let n = gs.field.n as i16;
    let bounded = |p: &Point| {
        let Point(i, j) = *p;
        if 0 <= i && i < m && 0 <= j && j < n { *p }
            else { Point(i.bound(0, m - 1), j.bound(0, n - 1)) }
    };
    for r in 1..(m + n) {
        for k in 0..r {
            let ps = [
                Point(oi - k, oj + r - k),
                Point(oi - r + k, oj - k),
                Point(oi + k, oj - r + k),
                Point(oi + r - k, oj + k),
            ];
            let opt = ps.iter().map(bounded).find(&predicate);
            if opt.is_some() {
                return opt;
            }
        }
    }
    None
}

fn direction(src: &Point, dst: &Point) -> Move {
    let Point(si, sj) = src;
    let Point(di, dj) = dst;
    if di == si && dj <= sj {
        Move::Left
    } else if di == si && dj > sj {
        Move::Right
    } else if di < si {
        Move::Up
    } else {
        Move::Down
    }
}
