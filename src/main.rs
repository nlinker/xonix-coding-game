pub mod utils;

use utils::Tr;

fn main() {
    let gs0 = r#"
            *.*.*.*.*.*.*.
            *. . . . . .*.
            *. . . . A .*.
            *. . . . B .*.
            *.*.*.*.*.*.*.
        "#;
    println!("trim_indent = {}", gs0.trim_indent());
}
