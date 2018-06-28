extern crate xcg;

#[cfg(test)]
mod test {

    use xcg::utils::Trim;

    #[test]
    fn indent_ops() {
        let gs0 = r#"
            aaa
           bbb
          ccc

        ddd
        "#;
        assert_eq!("    aaa\n   bbb\n  ccc\n\nddd", gs0.trim_indent());
        assert_eq!(">>>>    aaa\n>>>>   bbb\n>>>>  ccc\n>>>>\n>>>>ddd", gs0.replace_indent(">>>>"));
    }
}
