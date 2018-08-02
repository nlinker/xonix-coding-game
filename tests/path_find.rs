extern crate xcg;
extern crate priority_queue;

use priority_queue::PriorityQueue;
use std::collections::HashMap;

use xcg::utils::Trim;
use xcg::model::*;
use xcg::bot::common::W;
use xcg::bot::common::{P, a_star_find};
use xcg::bot::common::distance;

#[test]
fn test_a_star() {
    let mut gs = game_state(r#"
            *.*.*.*.*.*.*.*.*.*.*.
            *. . . . . . . . . .*.
            *. . . . . . . . . .*.
            *. . . . a a a . . .*.
            *. . . . a A a . . .*.
            *. . . . . . a . . .*.
            *. . a a a a a . . .*.
            *. . . . . . . . . .*.
            *.*.*.*.*.*.*.*.*.*.*.
        "#);

    let m = gs.field.m as i16;
    let n = gs.field.n as i16;
    let me = gs.players[0].body().iter().map(|p| P(p.1, m - 1 - p.0)).collect::<Vec<P>>();
    let is_boundary = |p: &P| {
        let P(x, y) = *p;
        0 <= y && y < m && 0 <= x && x < n && !me.contains(&p)
    };
    let heuristic = |p: &P, q: &P| distance(p, q);

    // we are using decartes coordinates, src -> dst
    let pairs = [
        (P(5, 4), P(9, 2)),
        (P(5, 4), P(8, 3))
    ];

    let mut gs_paths: Vec<String> = vec![];
    for (src, dst) in pairs.iter() {
        // clear previous path
        for i in 0..gs.field.m {
            for j in 0..gs.field.n {
                match gs.field.cells[i][j] {
                    Cell::Owned(_) => gs.field.cells[i][j] = Cell::Empty,
                    _ => (),
                }
            }
        }
        let path = {
            // use this logger for the debugging
            let mut _logger = Some(|ol: &PriorityQueue<P, W>, cl: &HashMap<P, P>| {
                for (k, _) in ol {
                    let P(x, y) = *k;
                    let j = x as usize;
                    let i = (m as usize) - 1 - (y as usize);
                    gs.field.cells[i][j] = Cell::Owned(0);
                }
                for (p, _) in cl {
                    let P(x, y) = *p;
                    let j = x as usize;
                    let i = (m as usize) - 1 - (y as usize);
                    gs.field.cells[i][j] = Cell::Owned(1);
                }
                println!("{}", prettify_game_state(&gs, false, false));
                println!("{:?}", ol);
            });
            let mut logger: Option<fn(&PriorityQueue<P, W>, &HashMap<P, P>)> = None;
            a_star_find(&src, &dst, is_boundary, heuristic, logger)
        };

        if let Some(path) = path {
            println!("path = {:?}", path);
            for P(x, y) in path {
                let j = x as usize;
                let i = (m as usize) - 1 - (y as usize);
                gs.field.cells[i][j] = Cell::Owned(2);
            }
            gs_paths.push(prettify_game_state(&gs, false, false));
        }
    }
    // (5,4) -> (9,2)
    let exp0 = r#"
        * * * * * * * * * * *
        * . . . . . . . . . *
        * . . . . . . . . . *
        * . . . a a a . . . *
        * . . . a A a . . . *
        * 2 2 2 2 2 a . . . *
        * 2 a a a a a 2 2 2 *
        * 2 2 2 2 2 2 2 . . *
        * * * * * * * * * * *
    "#.trim_indent();
    let exp0 = format!("A: 0                 \n{}\niteration: 0", exp0);
    assert_eq!(exp0, gs_paths[0]);

    // (5,4) -> (9,2)
    let exp1 = r#"
        * * * * * * * * * * *
        * . . . . . . . . . *
        * . . 2 2 2 2 2 . . *
        * . . 2 a a a 2 . . *
        * . . 2 a A a 2 . . *
        * . . 2 2 2 a 2 2 . *
        * . a a a a a . . . *
        * . . . . . . . . . *
        * * * * * * * * * * *
    "#.trim_indent();
    let exp1 = format!("A: 0                 \n{}\niteration: 0", exp1);
    assert_eq!(exp1, gs_paths[1]);
}


fn game_state(gs: &str) -> GameState {
    GameState::parse_string(&gs.trim_indent()).unwrap()
}
