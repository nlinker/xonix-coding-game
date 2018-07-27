use model::Move;
use utils::Bound;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

/// Decartes coordinates, (x, y)
/// make our own coordinate system, in the name of René Descartes
/// ^ y
/// |
/// |
/// +-------> x
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, PartialOrd)]
pub struct P(pub i16, pub i16);

pub fn distance(p: &P, q: &P) -> i16 {
    (p.0 - q.0).abs() + (p.1 - q.1).abs()
}

pub fn may_be_selected(base: &P, arrow: &P, cur: &P) -> bool {
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

pub fn direction(src: &P, dst: &P) -> Move {
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

pub fn build_path(src: &P, dst: &P, horz_first: bool) -> Vec<P> {
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

pub fn find_closest(m: i16, n: i16, src: &P, max: i16, predicate: impl Fn(&P) -> bool) -> Option<P> {
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

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Weight {
    f: i32,
    g: i32,
}

///
pub fn a_star_find(src: &P, dst: &P, cells: impl Fn(&P) -> bool) -> Option<Vec<P>> {

    let mut weight: HashMap<P, Weight> = HashMap::new();

    let mut queue: BinaryHeap<P> = BinaryHeap::new();

    let mut opened: HashMap<P, bool> = HashMap::new();
    let mut closed: HashMap<P, bool> = HashMap::new();

    weight[src] = Weight { f: 0, g: 0 };
    queue.push(*src);
    opened.push(*src, true);

    // while the open list is not empty
    while !queue.is_empty() {
        // pop the position of node which has the minimum `f` value.
        let node = queue.pop();
        closed.push(node, true);
        // if reached the end position, construct the path and return it
        if node == dst {
            return backtrace(dst);
        }

    }
    None
}

pub fn backtrace(dst: &P) -> Option<Vec<P>> {
    None
}