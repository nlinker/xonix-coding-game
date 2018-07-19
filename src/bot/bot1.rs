use model::Bot;
use model::Cell;
use model::GameState;
use model::Move;
use model::Point;
use rand::IsaacRng;
use rand::prelude::{FromEntropy, Rng};
use std::cell::RefCell;
use utils::Bound;

#[derive(Clone, Debug)]
pub struct Bot1 {
    idx: usize,
    random: RefCell<IsaacRng>,
    m: usize,
    n: usize,
    destination: Option<Point>,
    last_move: Move,
}

impl Bot for Bot1 {
    fn reset(&mut self, gs: &GameState, idx: u8, seed: u64) {
        self.m = gs.field.m;
        self.n = gs.field.n;
        self.idx = idx as usize;
        self.random = RefCell::new(IsaacRng::new_from_u64(seed));
    }

    fn do_move(&mut self, gs: &GameState) -> Move {
        let me = gs.players[self.idx].body();
        if me.is_empty() {
            return Move::Stop
        }
        let head = *gs.players[self.idx].head().unwrap(); // guaranteed to exist
        if self.last_move != Move::Stop {
            let (_, new_head) = self.calculate_heads(head, self.last_move);
            // don't try to select the last move, if it is to bite itself
            if me.contains(&new_head) {
                self.last_move = Move::Stop;
            }
        }
        let mut cur_move: Option<Move> = None;
        // some attempts to move
        for _ in 1..8 {
            if self.destination.is_none() {
                self.destination = self.calculate_destination(gs, head)
            } else {
                // probably reset if we achieved the destination
                // gs.field.cells[head] != Cell.Empty
                let p = self.destination.unwrap();
                if (p.0 - head.0).abs() + (p.0 - head.0).abs() <= 1 {
                    // try to find closest
                    // self.destination = self.find_closest(head, |ref p| gs.field.cells[p.0 as usize][p.1 as usize] != Cell::Empty);
                    self.destination = None;
                }
            }
            // now choose the move
            if self.destination.is_some() {
                let destination = self.destination.unwrap();
                let ri = destination.0 - head.0;
                let rj = destination.1 - head.1;
                let r = self.random.borrow_mut().gen_range(0, 4 + ri.abs() + rj.abs());
                let mv = if r < 4 {
                    select_move(r)
                } else if r < 4 + ri.abs() {
                    // vertical move
                    if ri < 0 { Move::Up } else { Move::Down }
                } else {
                    // horizontal move
                    if rj < 0 { Move::Left } else { Move::Right }
                };
                cur_move = Some(mv);
                let (_, new_head) = self.calculate_heads(head, cur_move.unwrap());
                if !me.contains(&new_head) {
                    break;
                }
            } else if self.last_move == Move::Stop {
                let r = self.random.borrow_mut().gen_range(0, 4);
                cur_move = Some(select_move(r));
                let (_, new_head) = self.calculate_heads(head, cur_move.unwrap());
                if !me.contains(&new_head) {
                    break;
                }
            } else {
                // higher probability to choose the last move
                let r = self.random.borrow_mut().gen_range(0, 16);
                let mv = if r < 4 { select_move(r) } else { self.last_move };
                cur_move = Some(mv);
                let (_, new_head) = self.calculate_heads(head, cur_move.unwrap());
                if !me.contains(&new_head) {
                    break;
                }
            }
        }
        self.last_move = cur_move.unwrap_or(Move::Stop);
        self.last_move
    }
}

impl Bot1 {
    pub fn new(idx: u8) -> Self {
        Bot1 {
            idx: idx as usize,
            random: RefCell::new(IsaacRng::from_entropy()),
            m: 0,
            n: 0,
            destination: None,
            last_move: Move::Stop
        }
    }

    fn calculate_heads(&self, old_head: Point, mv: Move) -> (Point, Point) {
        let m = self.m as i16;
        let n = self.n as i16;
        let (di, dj) = match mv {
            Move::Right => (0, 1),
            Move::Up    => (-1, 0),
            Move::Left  => (0, -1),
            Move::Down  => (1, 0),
            Move::Stop  => (0, 0),
        };
        let new_head = Point(
            (old_head.0 + di).bound(0, m - 1),
            (old_head.1 + dj).bound(0, n - 1)
        );
        (old_head, new_head)
    }

    fn calculate_destination(&self, gs: &GameState, head: Point) -> Option<Point> {
        // put several random dots into the field, and the first empty point
        // is our destination
        for _ in 1..16 {
            let i = self.random.borrow_mut().gen_range(0, self.m);
            let j = self.random.borrow_mut().gen_range(0, self.n);
            if gs.field.cells[i][j] == Cell::Empty {
                let p = Point(i as i16, j as i16);
                if p != head {
                    return Some(p);
                }
            }
        }
        // cannot choose the destination
        None
    }

//    fn find_closest(&self, src: Point, predicate: impl Fn(&Point) -> bool) -> Option<Point> {
//        let m = self.m as i16;
//        let n = self.n as i16;
//        let Point(oi, oj) = src;
//        let bounded = |p: &Point| {
//            let Point(i, j) = *p;
//            if 0 <= i && i < m && 0 <= j && j < n { *p }
//                else { Point(i.bound(0, m - 1), j.bound(0, n - 1)) }
//        };
//        for r in 1..(m + n) {
//            for k in 0..r {
//                let ps = [
//                    Point(oi - k, oj + r - k),
//                    Point(oi - r + k, oj - k),
//                    Point(oi + k, oj - r + k),
//                    Point(oi + r - k, oj + k),
//                ];
//                let opt = ps.iter().map(bounded).find(&predicate);
//                if opt.is_some() {
//                    return opt;
//                }
//            }
//        }
//        None
//    }
}

fn select_move(i: i16) -> Move {
    match i {
        0 => Move::Right,
        1 => Move::Up,
        2 => Move::Left,
        3 => Move::Down,
        4 => Move::Stop,
        _ => unreachable!(),
    }
}
