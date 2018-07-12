extern crate core;
extern crate rand;
extern crate itertools;
extern crate xcg;

#[cfg(test)]
mod test {
    use xcg::utils::Trim;
    use xcg::model::*;
    use xcg::test::TestBot;

    use rand::prelude::{Rng, RngCore};
    use rand::IsaacRng;

    use core::fmt::Debug;
    use std::collections::HashSet;
    use core::iter::FromIterator;
    use std::borrow::Borrow;
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_indent_ops() {
        let gs0 = r#"
            aaa
           bbb
          ccc

        ddd
        "#;
        assert_eq!("    aaa\n   bbb\n  ccc\n\nddd", gs0.trim_indent());
        assert_eq!(">>>>    aaa\n>>>>   bbb\n>>>>  ccc\n>>>>\n>>>>ddd", gs0.replace_indent(">>>>"));
    }

    #[test]
    fn test_border() {
        // test on the different cells sizes
        let field_sizes = vec![(2, 2), (9, 3), (3, 4), (4, 7)];
        //         n
        //   * * * * * * *
        // m *           *
        //   *           *
        //   * * * * * * *
        for (m, n) in field_sizes {
            let mut perimeter: Vec<Point> = vec![];
            perimeter.append(&mut points_2(0, 0..n));
            perimeter.append(&mut points_1(1..m - 1, n - 1));
            perimeter.append(&mut points_2(m - 1, (0..n).rev()));
            perimeter.append(&mut points_1((1..m - 1).rev(), 0));
            let size = 2 * (m + n - 2) as usize;
            assert_eq!(size, perimeter.len());
            for l in 0..=(2*size) {
                assert_eq!(perimeter[l % size], border_to_point(m as usize, n as usize, l as usize));
            }
        }
        fn points_1(ii: impl Iterator<Item=i16>, j: i16) -> Vec<Point> {
            ii.map(|i| Point(i, j)).collect()
        }
        fn points_2(i: i16, jj: impl Iterator<Item=i16>) -> Vec<Point> {
            jj.map(|j| Point(i, j)).collect()
        }
    }

    #[test]
    fn test_create_origins() {
        let m = 7;
        let n = 9;
        // 2 players - opposite corners
        let o2 = create_origins_n(m, n, 2);
        assert_eq!(vec![Point(0, 0), Point(6, 8)], o2);
        // 4 players - all corners
        let o4 = create_origins_n(m, n, 4);
        assert_eq!(vec![Point(0, 0), Point(6, 8), Point(0, 8), Point(6, 0)], o4);
        // otherwise - spread in the perimeter
        let o8 = create_origins_n(m, n, 8);
        assert_eq!(vec![
            Point(0, 0), Point(0, 3), Point(0, 6), Point(1, 8),
            Point(4, 8), Point(6, 7), Point(6, 4), Point(6, 1)
        ], o8);
    }

    #[test]
    fn test_permutations() {
        let perm0 = create_default_permutation(4);
        assert_eq!(vec![0, 1, 2, 3], perm0);
        let mut random = IsaacRng::new_from_u64(123);
        assert_eq!(vec![2, 0, 1, 3], copy_shuffled_permutation(&perm0, &mut random));
        assert_eq!(vec![3, 2, 1, 0], copy_shuffled_permutation(&perm0, &mut random));
        assert_eq!(vec![2, 1, 3, 0], copy_shuffled_permutation(&perm0, &mut random));
    }

    #[test]
    fn test_parse_string() {
        let str0 = r#"
            *.*.*.*.*A*a*a
            *.3d2.2.2.0.*a
            *.2D2.2C2.1.*.
            *.2.2. . .1B*.
            *.*.*.*.*.*b*b
            reordering=[2,1,3,0]
            stats=Stats(19,33,2,1,0,[1,2,9,1])
            origins=[(0,6),(4,6),(4,0),(0,0)]
        "#.trim_indent();
        let gs = GameState::parse_string(&str0[..]).unwrap();
        let str1 = gs.to_string();
        assert_eq!(str0, str1);
    }

    #[test]
    fn test_score() {
        let gs = game_state(r#"
            *.*.*.*.*.*.*.
            *.0. A1.1.1.*.
            *. a a B b2D*.
            *.3C3.3. .2.*.
            *.*.* *.*.*.*.
        "#);
        let stats = gs.stats;
        assert_eq!( vec![1, 3, 2, 3], stats.scores);
        assert_eq!( 29, stats.filled_count);
    }

    #[test]
    fn test_flood() {
        let gs: GameState = game_state(r#"
            *.*.*.*.*.*.*.
            *. .1. . . .*.
            *. a a a a A*.
            *. . .1. . .*.
            *.*.*.*.*.*.*B
        "#);
        let all: Vec<Point> = gs.players.iter()
            .flat_map(|p| p.0.clone())
            .collect();
        let bodies = HashSet::from_iter(all);
        let points1: HashSet<Point> = [].iter().cloned().collect();
        let points2: HashSet<Point> = [Point(1, 1)].iter().cloned().collect();
        let points3: HashSet<Point> = [Point(3, 5), Point(3, 4)].iter().cloned().collect();
        assert_eq!(points1, flood(&gs.field, &bodies, Point(2, 2)));
        assert_eq!(points2, flood(&gs.field, &bodies, Point(1, 1)));
        assert_eq!(points3, flood(&gs.field, &bodies, Point(3, 4)));
        let flooded_area = calculate_flood_area(&gs.field, &gs.players[0].0);
        assert_eq!(vec![Point(1, 1), Point(2, 1), Point(2, 2), Point(2, 3), Point(2, 4), Point(2, 5)], flooded_area)
    }

    #[test]
    fn test_flood_step() {
        let gs0 = game_state(r#"
            *.*.*.*.*.*.*.
            *. b B . A .*.
            *. a a a a .*.
            *. . . . . .*.
            *.*.*.*.*.*.*.
        "#);
        let a = test_bot("u");
        let gs1 = play(&gs0, &mut [a]);
        let mut gs2 = game_state(r#"
            *.*.*.*.*A*.*.
            *.0.0B0.0. .*.
            *.0.0.0.0. .*.
            *. . . . . .*.
            *.*.*.*.*.*.*.
        "#);
        gs2.stats.iteration = 1;
        assert_eq!(gs2.stats, gs1.stats);
        assert_eq!(gs2, gs1);
    }

    #[test]
    fn test_select_respawn() {
        // A's respawn is the upper left corner
        let gs0 = game_state(r#"
            *B*D*E*.*.*.*.
            *C . . . . .*.
            *. . . . A .*.
            *. . . . . .*.
            *.*.*.*.*.*.*.
        "#);
        let respawn = calculate_respawn(&gs0, 0);
        assert_eq!(respawn, Some(Point(2, 0)))
    }

    #[test]
    fn test_run_match_with_reordering() {
        let match_seed = Some(69);
        let random = Rc::new(RefCell::new(IsaacRng::new_from_u64(123)));

        let a = test_bot_r(0, random.clone(), "dlu");
        let b = test_bot_r(1, random.clone(), "llurr");
        let c = test_bot_r(2, random.clone(), "urd");
        let d = test_bot_r(3, random.clone(), "rrrdlll");
        let mut bots = [a, b, c, d];
        let names = bots.iter().map(|bot| bot.name()).collect::<Vec<String>>();

        let mut the_match = create_match(5, 7, &names, 20, 0.9, match_seed);
        let gs = game_state(r#"
            *C*.*.*.*.*.*B
            *. . . . . .*.
            *. . . . . .*.
            *. . . . . .*.
            *D*.*.*.*.*.*A
            reordering=[3,0,2,1]
            origins=[(4,6),(0,6),(0,0),(4,0)]
        "#);
        debug_assert_eq!(the_match.game_state, gs);
        let logger = |_gs: &GameState| {
            // println!("{}", gs)
        };
        run_match(&mut the_match, &mut bots, &logger);
        let final_gs = game_state(r#"
            *.*.*.*.*.*.*.
            *.2C2D3. .1.*B
            *. .3.3. . .*.
            *. . . . .0A*.
            *.*.*.*.*.*.*.
            reordering=[3,0,2,1]
            stats=Stats(20,27,2,0,0,[1,1,2,3])
            origins=[(4,6),(0,6),(0,0),(4,0)]
        "#);
        debug_assert_eq!(the_match.game_state, final_gs);
    }

    #[test]
    fn test_run_replay() {
        let random = Rc::new(RefCell::new(IsaacRng::new_from_u64(123)));
        let a = test_bot_r(0, random.clone(), "dllll");
        let b = test_bot_r(1, random.clone(), "luuuu");
        let c = test_bot_r(2, random.clone(), "urrrr");
        let d = test_bot_r(3, random.clone(), "rdddd");
        let mut bots = [a, b, c, d];
        let names = bots.iter().map(|bot| bot.name()).collect::<Vec<String>>();
        let logger = |_gs: &GameState| {};
        for _ in 0..100 {
            // run match
            let match_k_seed = (*random).borrow_mut().next_u64();
            let mut match_k = create_match(11, 11, &names, 32, 0.9, Some(match_k_seed));
            let replay_k = run_match(&mut match_k, &mut bots, &logger);
            let gs1 = run_replay(&replay_k, &logger);
            let gs2 = run_replay(&replay_k, &logger);
            debug_assert_eq!(match_k.game_state, gs1);
            debug_assert_eq!(match_k.game_state, gs2);
        }
    }

    #[test]
    fn test_run_tournament() {

    }


    // === helpers ===

    fn game_state(gs: &str) -> GameState {
        GameState::parse_string(&gs.trim_indent()).unwrap()
    }

    fn test_bot(path: &str) -> TestBot<IsaacRng> {
        TestBot::new(path)
    }

    fn test_bot_r<R: Rng>(idx: u8, rng: Rc<RefCell<R>>, path: &str) -> TestBot<R> {
        TestBot::with_index_random(path, idx, rng)
    }

    fn play<B: Bot>(gs: &GameState, bots: &mut [B]) -> GameState {
        let mut gs = gs.clone();
        let mut progressing = true;
        let mut iteration = 0;
        // reset bot's state
        for k in 0..bots.len() {
            let idx = gs.reordering[k];
            bots[idx as usize].reset(gs.borrow(), idx, 0u64);
        }
        // iterating
        while progressing {
            gs.stats.iteration = iteration;
            iteration += 1;
            let mut moves = vec![];
            for k in 0..bots.len() {
                let idx = gs.reordering[k];
                // let cgs = game.make_client_game_state(gs, idx);
                let mv = bots[idx as usize].do_move(gs.borrow());
                moves.push(mv);
                step(gs.borrow_mut(), idx, mv);
            }
            if moves.iter().all(|m| *m == Move::Stop) {
                progressing = false;
            }
        }
        gs
    }

    // http://play.rust-lang.org/?gist=ed56c0ea31c17399545386416af5b56c
    trait Nice {
        fn nice(&self) -> String;
    }

    impl<T: Debug> Nice for T {
        fn nice(&self) -> String {
            format!("{:?}", *self)
        }
    }

}
