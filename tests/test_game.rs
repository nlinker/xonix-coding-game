extern crate xcg;
extern crate rand;

extern crate core;
#[cfg(test)]
mod test {

    use xcg::utils::Trim;
    use xcg::model::*;
    use rand::IsaacRng;
    use core::fmt::Debug;
    use std::collections::HashSet;
    use core::iter::FromIterator;

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
            *. . .1. . .*.
            *a a a a a A*.
            *. .1. . . .*.
            *.*.*.*.*.*.*B
        "#);
        let all: Vec<Point> = gs.players.iter()
            .flat_map(|p| p.0.clone())
            .collect();
        let bodies = HashSet::from_iter(all);
        let points1: HashSet<Point> = [Point(1, 1), Point(1, 2)].iter().cloned().collect();
        let points2: HashSet<Point> = [].iter().cloned().collect();
        let points3: HashSet<Point> = [Point(3, 3), Point(3, 4), Point(3, 5)].iter().cloned().collect();
        assert_eq!(points1, flood(&gs.field, &bodies, Point(1, 1)));
        assert_eq!(points2, flood(&gs.field, &bodies, Point(3, 2)));
        assert_eq!(points3, flood(&gs.field, &bodies, Point(3, 3)));
    }


    fn game_state(gs: &str) -> GameState {
        GameState::parse_string(&gs.trim_indent()).unwrap()
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
