extern crate rand;
extern crate xcg;
extern crate console;

use rand::IsaacRng;
use rand::prelude::RngCore;
use std::cell::RefCell;
use console::style;

use xcg::model::*;
use xcg::bot::Bot1;
use xcg::bot::Bot2;
use std::thread;
use std::time::Duration;

fn main() {
    println!("This is {} neat", style("quite").cyan());
}

fn main1() {
//    let mut buf = [0; 16];
//    {
//        let (mut b1, mut b2) = buf.split_at_mut(8);
//        byteorder::LittleEndian::write_u64(&mut b1, 123);
//        byteorder::LittleEndian::write_u64(&mut b2, 123);
//    }
//    let random = Rc::new(RefCell::new(XorShiftRng::from_seed(buf)));
    let random = RefCell::new(IsaacRng::new_from_u64(234));
    let m = 32;
    let n = 48;
    let timeout = 30;

    let a = Bot1::new(0);
    let b = Bot1::new(1);
    let c = Bot2::new(2);
    let d = Bot2::new(3);
//    let mut bots: [Box<dyn Bot>; 1] = [Box::new(d)];
    let mut bots: [Box<dyn Bot>; 4] = [Box::new(a), Box::new(b), Box::new(c), Box::new(d)];
    let names: Vec<String> = bots.iter().enumerate()
        .map(|(k, _)| ((('A' as u8) + (k as u8)) as char).to_string())
        .collect();

    let logger = |gs: &GameState| {
        if gs.stats.iteration > 0 {
            println!("{}", prettify_game_state(gs, true, true));
            thread::sleep(Duration::from_millis(timeout));
        }
    };
    for _it in 0..100 {
        // run match
        let match_k_seed = random.borrow_mut().next_u64();
        let mut match_k = create_match(m, n, &names, 1024, 0.9, Some(match_k_seed));
        let _replay_k = run_match(&mut match_k, &mut bots, &logger);
        println!("{} {:?}", "\n".repeat(m + names.len()), match_k.game_state.stats);
    }
}
