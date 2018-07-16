#![allow(unused)]

extern crate rand;

use rand::prelude::Rng;
use model::Move;
use model::Point;
use std::cell::RefCell;
use model::GameState;
use model::Cell;
use utils::Bound;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Bot2<R: Rng> {
    index: u8,
    m: usize,
    n: usize,
    random: RefCell<R>,
    destination: Option<Point>,
    last_move: Option<Move>,
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

fn direction(src: Point, dst: Point) -> Move {
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
