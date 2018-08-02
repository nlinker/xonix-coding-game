use model::Move;
use utils::Bound;
use std::cmp::Ordering;
use priority_queue::PriorityQueue;
use std::fmt;
use std::fmt::Write;
use std::collections::HashMap;

/// Decartes coordinates, (x, y)
/// make our own coordinate system, in the name of René Descartes
/// ^ y
/// |
/// |
/// +-------> x
#[derive(Clone, Copy, Eq, PartialEq, Hash, PartialOrd)]
pub struct P(pub i16, pub i16);

impl Ord for P {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 {
            Ordering::Less
        } else if self.0 > other.0 {
            Ordering::Greater
        } else {
            self.1.cmp(&other.1)
        }
    }
}

impl fmt::Debug for P {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_char('(')?;
        fmt.write_str(&self.0.to_string())?;
        fmt.write_char(',')?;
        fmt.write_str(&self.1.to_string())?;
        fmt.write_char(')')?;
        Ok(())
    }
}

impl fmt::Display for P {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_char('(')?;
        fmt.write_str(&self.0.to_string())?;
        fmt.write_char(',')?;
        fmt.write_str(&self.1.to_string())?;
        fmt.write_char(')')?;
        Ok(())
    }
}

pub fn distance(p: &P, q: &P) -> i32 {
    ((p.0 - q.0).abs() as i32) + ((p.1 - q.1).abs() as i32)
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
    for r in 1..max {
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

const NEIGHBORS: &[(i16, i16)] = &[(0, -1), (-1, 0), (0, 1), (1, 0)];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct W {
    // h: f32, // heuristic distance from the end node
    pub f: i32, // g + h
    pub g: i32, // distance from the starting node
    pub par: P, // the point where we came from
}

impl PartialOrd for W {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.f.cmp(&other.f).reverse())
    }
}

impl Ord for W {
    fn cmp(&self, other: &Self) -> Ordering {
        // partial_cmp for W is never None
        self.partial_cmp(&other).unwrap()
    }
}


// for debug you can add
pub fn a_star_find(src: &P, dst: &P,
                   is_accessible: impl Fn(&P) -> bool,
                   heuristic: impl Fn(&P, &P) -> i32,
                   mut logger: Option<impl FnMut(&PriorityQueue<P, W>, &HashMap<P, P>) -> ()>,
) -> Option<Vec<P>> {
    let mut open_list: PriorityQueue<P, W> = PriorityQueue::new();
    let mut closed_list: HashMap<P, P> = HashMap::new();
    // 1. Take the start node and put it on the open list
    open_list.push(*src, W {f: 0, g: 0, par: *src});
    // 2. While there are nodes in the open list:
    while !open_list.is_empty() {
        // 3. Pick the node from the open list having the smallest `f` score.
        // Put it on the closed list (you don't want to consider it again).
        let (cur_p, cur_w) = open_list.pop().unwrap();
        closed_list.insert(cur_p, cur_w.par);
        // 4. if reached the end position, construct the path and return it
        if cur_p == *dst {
            return backtrace(&closed_list, *dst);
        }
        // 5. For each neighbor (adjacent cell) which isn't in the closed list:
        //   a. Set its parent to current node.
        //   b. Calculate `g` score (distance from starting node to this neighbor) and add it to the open list
        //   c. Calculate `f` score by adding heuristics to the `g` value.
        let accessible_neigh = NEIGHBORS.iter()
            .map(|(dx, dy)| P(cur_p.0 + dx, cur_p.1 + dy))
            .filter(|p| !closed_list.contains_key(&p) && is_accessible(&p));
        for np in accessible_neigh {
            // the neighbour could be already accessible from the different node
            let g = cur_w.g + if np != cur_p { 1 } else { 0 };
            let f = g + heuristic(&np, &dst);
            let par = cur_p;
            let mut w_opt = open_list.get_priority(&np).map(|w| w.clone());
            match w_opt {
                Some(w) => {
                    if g < w.g {
                        // the neighbour can be reached with smaller cost
                        open_list.change_priority(&np, W { f, g, par });
                    };
                    // otherwise don't touch the neighbour, it will be taken by open_list.pop()
                },
                None => {
                    // the neighbour is the new
                    open_list.push(np, W { f, g, par });
                },
            };
        }
        if let Some(ref mut log) = logger {
            log(&open_list, &closed_list);
        }
    }
    None
}

pub fn backtrace(closed_list: &HashMap<P, P>, dst: P) -> Option<Vec<P>> {
    let mut p = dst;
    let mut result = vec![p];
    while let Some(parent) = closed_list.get(&p) {
        result.push(*parent);
        if p == *parent {
            // for src we have src.par = src
            break;
        }
        p = *parent;
    }
    result.reverse();
    Some(result)
}
