extern crate xcg;
extern crate rand;

#[cfg(test)]
mod test {

    use xcg::utils::Trim;
    use xcg::model::*;
    use rand::IsaacRng;

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
          *.*.*.*.*A*.*.
          *.3.2.2.2.0.*.
          *.2D2.2C2.1.*.
          *.2.2. . .1B*.
          *.*.*.*.*.*.*.
          reordering=[2,1,3,0]
          stats=Stats(19,33,2,1,0,[1,2,9,1])
          origins=[(0,6),(4,6),(4,0),(0,0)]
        "#.trim_indent();
        let gs = GameState::parse_string(&str0[..]);
        println!("\n-----------\n{:?}", gs)
    }

//    testParseString() {
//val seed = 42L;
//val random = new Random(96);
//List<Bot> bots = IntStream.range(0, 4)
//.mapToObj(i -> new TestBot(i, "", random))
//.collect(toList());
//Supplier<List<Bot>> botFactory = () -> bots;
//val match = game.createMatch(5, 7, botFactory, 20, 90.0, Optional.of(seed));
//MatchLogger logger = (gs, ns) -> {};
//game.runMatch(match, logger);
//String str = match.getGameState().toString();
//assertEquals("" +
//"*.*.*.*.*A*.*.\n" +
//"*.3.2.2.2.0.*.\n" +
//"*.2D2.2C2.1.*.\n" +
//"*.2.2. . .1B*.\n" +
//"*.*.*.*.*.*.*.\n" +
//"reordering=[2,1,3,0]\n" +
//"stats=Stats(19,33,2,1,0,[1,2,9,1])\n" +
//"origins=[(0,6),(4,6),(4,0),(0,0)]\n", str);
//val gs2 = gameState(str);
//assertEquals(match.getGameState(), gs2);
//}

}
