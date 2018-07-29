extern crate xcg;

use xcg::utils::Trim;
use xcg::model::*;
use xcg::bot::common::{P, a_star_find};

#[test]
fn test_a_star() {
    let gs = game_state(r#"
        *.*.*.*.*.*.*.*.*.*.*.
        *. . . . . . . . . .*.
        *. . . . . . . . . .*.
        *. . . . a a a . . .*.
        *. B . . a A a . . .*.
        *. b . . . . a . . .*.
        *. b a a a a a . . .*.
        *. b . . . . . . . .*.
        *.*.*.*.*.*.*.*.*.*.*.
    "#);
    // decartes coordinates
    let src = P(4, 4);
    let m = gs.field.m as i16;
    let n = gs.field.n as i16;
    let me = gs.players[0].body().iter().map(|p| P(p.1, m - 1 - p.0)).collect::<Vec<P>>();
    let is_boundary = |p: &P| {
        let P(x, y) = *p;
        0 <= y && y < m && 0 <= x && x < n && me.contains(&p)
    };
    let dst = P(9, 1);
    let path = a_star_find(&src, &dst, is_boundary);
    println!("{:?}", path);
}


fn game_state(gs: &str) -> GameState {
    GameState::parse_string(&gs.trim_indent()).unwrap()
}
