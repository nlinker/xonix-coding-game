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
use std::rc::Rc;

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
    path_pos: usize,
}

struct Bot2Alg<'a> {
    gs: &'a GameState,
    random: Rc<RefCell<IsaacRng>>,
}

impl Bot for Bot2 {
    fn reset(&mut self, gs: &GameState, idx: u8, seed: u64) {
        self.idx = idx as usize;
        self.m = gs.field.m;
        self.n = gs.field.n;
        self.random = Rc::new(RefCell::new(IsaacRng::new_from_u64(seed)));
    }

    fn do_move(&mut self, gs: &GameState) -> Move {

        let alg = Bot2Alg { gs, random: self.random.clone() };

        self.last_me = self.cur_me.clone();
        self.cur_me = alg.player(self.idx);
        if self.cur_me.is_empty() {
            return Move::Stop;
        }
        let cur_head = self.cur_me.first().unwrap();
        // if we were flooded or bitten, then reset the path
        if self.cur_me.len() < self.last_me.len() {
            self.path = vec![];
            self.path_pos = 0;
        }

        let the_move = if !self.path.is_empty() {
            if self.path_pos < self.path.len() - 1 {
                self.path_pos += 1;
            } else {
                self.path_pos = 0;
            }
            let new_head = self.path.first().unwrap();
            direction(cur_head, new_head)
        } else {
            // generate the new path
            let mut empties = alg.find_random_empty(20);
            &empties.sort_by_key(|p| distance(cur_head, p));
            // now try to take approximately 5th element
            let the_empty = &empties.take(100).last();
            eprintln!("the_empty = {:?}", the_empty);
            Move::Stop
        };

        the_move
    }
}

impl<'a> Bot2Alg<'a> {
    /// the head is the _first_ element, as opposite to `self.gs.players[idx]`
    fn player(&self, idx: usize) -> Vec<P> {
        self.gs.players[idx].body().iter().rev().map(|p| self.to_decartes(p)).collect()
    }

    fn find_closest(&self, src: P, predicate: impl Fn(&P) -> bool) -> Option<P> {
        let P(xs, ys) = src;
        let m = self.gs.field.m as i16;
        let n = self.gs.field.n as i16;
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
    fn border_or_owned_partial(&self, o: P, a: P, c: P) -> bool {
        let cell = self.cells(&c);
        (cell != Cell::Empty) && may_be_selected(o, a, c)
    }

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
            path_pos: 0,
        }
    }
}

fn distance(p: &P, q: &P) -> i16 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

fn may_be_selected(base: P, arrow: P, cur: P) -> bool {
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
        Move::Left
    } else if dx == sx && dy > sy {
        Move::Right
    } else if dx < sx {
        Move::Up
    } else {
        Move::Down
    }
}
