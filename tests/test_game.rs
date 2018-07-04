extern crate xcg;
extern crate rand;

extern crate core;

#[cfg(test)]
mod test {

    use xcg::utils::Trim;
    use xcg::model::*;
    use rand::IsaacRng;
    use core::fmt::Debug;

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
        let gs = GameState::parse_string(&str0[..]);
        println!("\n-----------\n{:?}", gs)
    }

    #[test]
    fn test_permutations() {
        let perm0 = create_default_permutation(4);
        assert_eq!("[0, 1, 2, 3]", perm0.nice());
        let mut random = IsaacRng::new_from_u64(123);
        assert_eq!("[2, 0, 1, 3]", copy_shuffled_permutation(&perm0, &mut random).nice());
        assert_eq!("[3, 2, 1, 0]", copy_shuffled_permutation(&perm0, &mut random).nice());
        assert_eq!("[2, 1, 3, 0]", copy_shuffled_permutation(&perm0, &mut random).nice());
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
