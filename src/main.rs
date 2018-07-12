#![allow(unused)]

extern crate rand;
extern crate byteorder;
extern crate xcg;
extern crate regex;
extern crate itertools;

use rand::IsaacRng;
use std::fmt;
use std::cell::RefCell;
use rand::prelude::Rng;
use std::rc::Rc;
use xcg::model::*;
use xcg::test::TestBot;
use xcg::utils::Trim;

fn main() {
    let teh_rng = Rc::new(RefCell::new(IsaacRng::new_from_u64(666)));

    let a = test_bot_r(0, teh_rng.clone(), "dlu");
    let b = test_bot_r(1, teh_rng.clone(), "llurr");
    let c = test_bot_r(2, teh_rng.clone(), "urd");
    let d = test_bot_r(3, teh_rng.clone(), "rrrdlll");
    let mut bots = [a, b, c, d];
    let names: Vec<String> = bots.iter().map(|bot| bot.name()).collect();

    let mut the_match = create_match(5, 7, &names, 20, 0.9, Some(69));
    let gs = game_state(r#"
            *C*.*.*.*.*.*B
            *. . . . . .*.
            *. . . . . .*.
            *. . . . . .*.
            *D*.*.*.*.*.*A
            reordering=[3,0,2,1]
            origins=[(4,6),(0,6),(0,0),(4,0)]
        "#);
    eprintln!("gs = \n{}", the_match.game_state);
    assert_eq!(gs, the_match.game_state);

    let logger: Box<Fn(&GameState)> = Box::new(|gs| {
        println!("{}", gs)
    });
    run_match(&mut the_match, &mut bots, logger);
    let mut final_gs = game_state(r#"
            *.*.*.*.*.*A*.
            *D3.3.3. .0.*.
            *. . . . . .*.
            *.2. . .1.1.*B
            *.*C*.*.*.*.*.
            reordering=[3,0,2,1]
            origins=[(4,6),(0,6),(0,0),(4,0)]
            stats=Stats(20,27,0,0,0,[1,2,1,3])
        "#);
    assert_eq!(the_match.game_state.to_string(), final_gs.to_string());
    assert_eq!(the_match.game_state, final_gs);
}

fn test_bot_r<R: Rng>(idx: u8, rng: Rc<RefCell<R>>, path: &str) -> TestBot<R> {
    TestBot::with_index_random(path, idx, rng)
}

fn game_state(gs: &str) -> GameState {
    GameState::parse_string(&gs.trim_indent()).unwrap()
}
