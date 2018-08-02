use bot::common::build_path;
use bot::common::direction;
use bot::common::distance;
use bot::common::find_closest;
use bot::common::may_be_selected;
use bot::common::P;
use core::cmp;
use model::Bot;
use model::Cell;
use model::GameState;
use model::Move;
use model::Point;
use rand::IsaacRng;
use rand::prelude::{FromEntropy, Rng};
use std::cell::RefCell;
use std::rc::Rc;

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
    chasing: bool, // if the bot is trying to bite someone now
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
            chasing: false,
        }
    }
}

impl Bot for Bot2 {
    fn reset(&mut self, gs: &GameState, idx: u8, seed: u64) {
        // must be like self.* = Bot2::new(idx).*;
        *self = Bot2::new(idx);
        self.random = Rc::new(RefCell::new(IsaacRng::new_from_u64(seed)));
        self.m = gs.field.m;
        self.n = gs.field.n;
    }

    fn do_move(&mut self, gs: &GameState) -> Move {

        let alg = Bot2Alg { gs, random: self.random.clone() };
//        let m = gs.field.m as i16;
//        let n = gs.field.n as i16;
//        let np = gs.players.len();
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
//        let mut enemy: Option<P> = None;
//        for k in 0..np {
//            if k != self.idx {
//                enemy = find_closest(m, n, cur_head, radius, |p| self.all[k].contains(p));
//                if enemy.is_some() {
//                     break;
//                }
//            }
//        }

        if let Some(_enemy) = alg.find_enemy_nearby(cur_head, radius) {
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
                if let Some(border) = alg.find_closest_on_field(the_empty, |ref p| alg.border_or_owned_partial(cur_head, the_empty, p)) {
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
        find_closest(m, n, src, m + n, predicate)
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

    /// find a
    fn find_safe_path() -> Option<Vec<P>> {
        None
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

    fn find_enemy_nearby(&self, _o: &P, _radius: i16) -> Option<Point> {
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
        // let n = self.gs.field.n as i16;
        P(p.1, m - 1 - p.0)
    }
}


