
pub mod utils;
use utils::trim_indent;

fn main() {
    let gs0 = r#"
        *B*D*E*.*.*.*.
        *C . . . . .*.
        *. . . . A .*.
        *. . . . . .*.
        *.*.*.*.*.*.*.
    "#;
    println!("trim_indent = {}", trim_indent(&gs0))
}
