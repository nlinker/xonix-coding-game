#![allow(unused)]

use model::Bot;
use model::Cell;
use model::GameState;
use model::Move;
use model::Point;
use rand::IsaacRng;
use rand::prelude::{Rng, FromEntropy};
use core::cmp;
use utils::Bound;
use std::rc::Rc;
use std::cell::RefCell;

/// Decartes coordinates, (x, y)
/// make our own coordinate system, in the name of René Descartes
/// ^ y
/// |
/// |
/// +-------> x
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, PartialOrd)]
struct P(i16, i16);

#[derive(Clone, Debug)]
pub struct Bot2 {
    idx: usize,
    random: Rc<RefCell<IsaacRng>>,
    m: usize,
    n: usize,
    cur_me: Vec<P>,
    last_me: Vec<P>,
    path: Vec<P>,
    path_idx: usize,
    stay_count: i32,
    all: Vec<Vec<P>>,
}

struct Bot2Alg<'a> {
    gs: &'a GameState,
    random: Rc<RefCell<IsaacRng>>,
}

impl Bot2 {
    pub fn new(idx: u8) -> Self {
        Bot2 {
            idx: idx as usize,
            random: Rc::new(RefCell::new(IsaacRng::from_entropy())),
            m: 0,
            n: 0,
            cur_me: vec![],
            last_me: vec![],
            path: vec![],
            path_idx: 0,
            stay_count: 0,
            all: vec![],
        }
    }
}

impl Bot for Bot2 {
    fn reset(&mut self, gs: &GameState, idx: u8, seed: u64) {
        // must be like self.* = Bot2::new(idx).*;
        self.idx = idx as usize;
        self.random = Rc::new(RefCell::new(IsaacRng::new_from_u64(seed)));
        self.m = gs.field.m;
        self.n = gs.field.n;
        self.cur_me = vec![];
        self.last_me = vec![];
        self.path = vec![];
        self.path_idx = 0;
        self.stay_count = 0;
        self.all = vec![];
    }

    fn do_move(&mut self, gs: &GameState) -> Move {

        let alg = Bot2Alg { gs, random: self.random.clone() };

        let np = gs.playesr.len();
        self.last_me = self.cur_me.clone();
        let (cur_me, all) = alg.player_bodies(self.idx);
        self.all = all;
        self.cur_me = cur_me;
        if self.cur_me.is_empty() {
            return Move::Stop;
        }
        let cur_head = self.cur_me.last().unwrap();
        // if we were flooded or bitten, then reset the path
        if self.cur_me.len() < self.last_me.len() {
            self.path = vec![];
            self.path_idx = 0;
        }

        // if we have found someone near, bite him
        let radius = self.random.borrow_mut().gen_range(2, 4);
        let mut enemy: Option<P> = None;
        for k in 0..np {
            if k != self.idx {
                enemy = find_closest(cur_head, radius, |p| self.all[k].contains(p));
                if enemy.is_some() {
                     break;
                }
            }
        }

        if let Some(enemy) = alg.find_enemy_nearby(cur_head, radius) {
            // change the path so that we will attempt to bite and then return back
        }

        let the_move = if !self.path.is_empty() && self.path_idx < self.path.len() {
            let new_head = self.path[self.path_idx];
            // in principle we could bump someone's head and the previous move had no effect,
            // in this case don't advance the position
            if self.cur_me != self.last_me {
                self.path_idx += 1;
                self.stay_count = 0;
            } else {
                self.stay_count += 1;
                if self.stay_count > 3 {
                    self.path_idx = 0;
                    self.stay_count = 0;
                    self.path.clear();
                }
            }
            direction(cur_head, &new_head)
        } else {
            // generate the new path
            let mut empties = alg.find_random_empty(20);
            &empties.sort_by_key(|p| distance(cur_head, p));
            // we have a vector of empty cells,
            // now try to take approximately 5th element
            if let Some(the_empty) = empties[..cmp::min(4, empties.len())].last() {
                let the_direction = direction(cur_head, the_empty);
                let mut path = build_path(cur_head, the_empty, the_direction == Move::Left || the_direction == Move::Right);
                if let Some(border) = alg.find_closest(the_empty, |ref p| alg.border_or_owned_partial(cur_head, the_empty, p)) {
                    let horz_first = self.random.borrow_mut().gen();
                    let mut appendix = build_path(the_empty, &border, horz_first);
                    path.append(&mut appendix);
                }
                self.path = path;
                if self.path.is_empty() {
                    self.path_idx = 0;
                    Move::Stop
                } else {
                    // path is non-empty
                    let new_head = &self.path[0];
                    self.path_idx = 1;
                    direction(cur_head, new_head)
                }
            } else {
                // we couldn't find an empty destination this time
                Move::Stop
            }
        };

        the_move
    }
}

impl<'a> Bot2Alg<'a> {
    /// the head is the _last_ element, similar to `self.gs.players[idx]`
    fn player_bodies(&self, idx: usize) -> (Vec<P>, Vec<Vec<P>>) {
        let me = self.gs.players[idx].body().iter().map(|p| self.to_decartes(p)).collect();
        let mut all: Vec<Vec<P>> = vec![];
        for k in 0..self.gs.players.len() {
            all.push(self.gs.players[k].body().iter().map(|p| self.to_decartes(p)).collect())
        }
        (me, all)
    }

    fn find_closest_on_field(&self, src: &P, predicate: impl Fn(&P) -> bool) -> Option<P> {
        let m = self.gs.field.m as i16;
        let n = self.gs.field.n as i16;
        find_closest(src, m + n, predicate)
    }

    fn find_random(&self, attempts: usize, predicate: impl Fn(&P) -> bool) -> Vec<P> {
        let m = self.gs.field.m as i16;
        let n = self.gs.field.n as i16;
        let mut buf: Vec<P> = Vec::with_capacity(attempts);
        for _ in 0..attempts {
            let x = self.random.borrow_mut().gen_range(0, n as i16);
            let y = self.random.borrow_mut().gen_range(0, m as i16);
            let p = P(x, y);
            if predicate(&p) {
                buf.push(p)
            }
        }
        buf
    }

    fn find_random_empty(&self, attempts: usize) -> Vec<P> {
        self.find_random(attempts, |p| self.cells(&p) == Cell::Empty)
    }

    /// to close the path we are interested in not any border or owned,
    /// but we need to find such cell, direction to that will not cross our body
    fn border_or_owned_partial(&self, o: &P, a: &P, c: &P) -> bool {
        let cell = self.cells(&c);
        (cell != Cell::Empty) && may_be_selected(o, a, c)
    }

    fn find_enemy_nearby(&self, o: &P, radius: i16) -> Option<Point> {
//        let m = self.gs.field.m;
//        let from_decartes_x = |x: i16| x as usize;
//        let from_decartes_y = |y: i16| m - 1 - (y as usize);
//        let is_enemy = |p: &P| {};
        None
    }

    // === helpers //

    fn cells(&self, p: &P) -> Cell {
        let m = self.gs.field.m;
        let from_decartes_x = |x: i16| x as usize;
        let from_decartes_y = |y: i16| m - 1 - (y as usize);
        self.gs.field.cells[from_decartes_y(p.1)][from_decartes_x(p.0)]
    }

    fn to_decartes(&self, p: &Point) -> P {
        let m = self.gs.field.m as i16;
        let n = self.gs.field.n as i16;
        P(p.1, m - 1 - p.0)
    }
}

fn distance(p: &P, q: &P) -> i16 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn may_be_selected(base: &P, arrow: &P, cur: &P) -> bool {
    let P(xb, yb) = base;
    let P(xa, ya) = arrow;
    let P(xc, yc) = cur;
    // 4 3 2
    // 5 9 1
    // 6 7 8
    if false { false }
    else if xb == xa && yb < ya { ya <= yc }
    else if xb > xa && yb < ya  { xc <= xa && ya <= yc }
    else if xb > xa && yb == ya { xc <= xa }
    else if xb > xa && yb > ya  { xc <= xa && yc <= ya }
    else if xb == xa && yb > ya { yc <= ya }
    else if xb < xa && yb > ya  { xa <= xc && yc <= ya }
    else if xb < xa && yb == ya { xa <= xc }
    else if xb < xa && yb < ya  { xa <= xc && ya <= yc }
    else if xb == xa && yb < ya { ya <= yc }
    else                        { xa != xc && ya != yc }
}

fn direction(src: &P, dst: &P) -> Move {
    let P(sx, sy) = src;
    let P(dx, dy) = dst;
    if dx == sx && dy <= sy {
        Move::Down
    } else if dx == sx && dy > sy {
        Move::Up
    } else if dx < sx {
        Move::Left
    } else {
        Move::Right
    }
}

fn build_path(src: &P, dst: &P, horz_first: bool) -> Vec<P> {
    fn h(y: i16, a: i16, b: i16) -> Vec<P> {
        if a < b { ((a + 1)..=b).map(|x| P(x, y)).collect() }
            else if b < a { (b..a).map(|x| P(x, y)).rev().collect() }
                else { vec![] }
    }
    fn v(x: i16, a: i16, b: i16) -> Vec<P> {
        if a < b { ((a + 1)..=b).map(|y| P(x, y)).collect() }
            else if b < a { (b..a).map(|y| P(x, y)).rev().collect() }
                else { vec![] }
    }
    let P(xs, ys) = src;
    let P(xd, yd) = dst;
    let mut path = vec![];
    if horz_first {
        // do ← → then ↑ ↓
        path.append(&mut h(*ys, *xs, *xd));
        path.append(&mut v(*xd, *ys, *yd));
    } else {
        // do ↑ ↓ then ← →
        path.append(&mut v(*xs, *ys, *yd));
        path.append(&mut h(*yd, *xs, *xd));
    };
    path
}

fn find_closest(src: &P, max: i16, predicate: impl Fn(&P) -> bool) -> Option<P> {
    let P(xs, ys) = src;
    let bounded = |p: &P| {
        let P(x, y) = *p;
        if 0 <= x && x < n && 0 <= y && y < m { *p }
            else { P(x.bound(0, n - 1), y.bound(0, m - 1)) }
    };
    for r in 1..(m + n) {
        for k in 0..r {
            let ps = [
                P(xs - k, ys + r - k),
                P(xs - r + k, ys - k),
                P(xs + k, ys - r + k),
                P(xs + r - k, ys + k),
            ];
            let opt = ps.iter().map(bounded).find(&predicate);
            if opt.is_some() {
                return opt;
            }
        }
    }
    None
}
