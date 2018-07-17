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
pub struct Bot2<'a> {
    idx: usize,
    random: RefCell<IsaacRng>,
    m: usize,
    n: usize,
    cur_me: Vec<Point>,
    last_me: Vec<Point>,
    path: &'a [Point],
//     gs: Option<&'a GameState>
}

impl<'a> Bot for Bot2<'a> {
    fn reset(&mut self, gs: &GameState, idx: u8, seed: u64) {
        self.idx = idx as usize;
        self.m = gs.field.m;
        self.n = gs.field.n;
        self.random = RefCell::new(IsaacRng::new_from_u64(seed));
    }

    fn do_move(&mut self, gs: &GameState) -> Move {
        let default_path = vec![];
        self.last_me = self.cur_me.clone();
        self.cur_me = *gs.players[self.idx].body();
        if self.cur_me.is_empty() {
            return Move::Stop;
        }
        let cur_head = gs.players[self.idx].head().unwrap();
        // if we were flooded or bitten, then reset the path
        if self.cur_me.len() < self.last_me.len() {
            self.path = &default_path;
        }

        let the_move = if !self.path.is_empty() {
            let new_head = self.path.first().unwrap();
            self.path = &self.path[1..];
            direction(cur_head, new_head)
        } else {
            Move::Stop // TODO
        };

        the_move
    }
}

impl<'a> Bot2<'a> {
    pub fn new(idx: u8) -> Self {
        Bot2 {
            idx: idx as usize,
            random: RefCell::new(IsaacRng::from_entropy()),
            m: 0,
            n: 0,
            cur_me: vec![],
            last_me: vec![],
            path: &vec![],
        }
    }

    /// Note: Decartes coordinates to accept
    fn cells(gs: &GameState, p: Point) -> Cell {
        let from_x = |x: i16| x as usize;
        let from_y = |y: i16| gs.field.m - 1 - (y as usize);
        gs.field.cells[from_y(p.1)][from_x(p.0)]
    }

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
