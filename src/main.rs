
pub mod utils;
use utils::trim_indent;
// use utils::get_add_function;

fn main() {
    let s = "aaa".trim_indent();
    println!("{}", s);
    let gs0 = r#"
        *B*D*E*.*.*.*.
        *C . . . . .*.
        *. . . . A .*.
        *. . . . . .*.
        *.*.*.*.*.*.*.
    "#;
    println!("trim_indent = {}", trim_indent(&gs0));
}

trait Tr {
    fn trim_indent(self) -> Self;
}

impl<'a> Tr for &'a str {
    fn trim_indent(self) -> Self {
        &self[..1] // some transformation here
    }
}
