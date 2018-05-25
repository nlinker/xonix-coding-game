pub mod utils;

use utils::Trim;

fn main() {
    let gs0 = r#"
            aaa
           bbb
          ccc

        ddd
        "#;
    println!("trim_indent = \n{}", gs0.trim_indent());
    println!("replace_indent = \n{}", gs0.replace_indent(">>>>"));
}
