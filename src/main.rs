
pub mod utils;
use utils::trim_indent;
// use utils::get_add_function;

fn main() {
    let gs0 = r#"
        *B*D*E*.*.*.*.
        *C . . . . .*.
        *. . . . A .*.
        *. . . . . .*.
        *.*.*.*.*.*.*.
    "#;
    println!("{}", make_adder(1)(2));
    println!("{}", make_adder(2)(3));
    let indent = "xxx".to_ascii_lowercase();
    let f = make_str_adder(&indent);
    println!("add = {}", f("_abc_"));
    println!("trim_indent = {}", trim_indent(&gs0))
}

fn make_str_adder<'a>(a: &'a str) -> Box<Fn(&str) -> String + 'static> {
    if a.chars().all(|c| c.is_ascii_whitespace()) {
        Box::new(move |l| String::from(l))
    } else {
        Box::new(move |l| {
            let mut r = String::new();
            r.push_str(l);
            r.push_str(a);
            r
        })
    }
}

fn make_adder(a: i32) -> Box<Fn(i32) -> i32> {
    if a == 1 {
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a * b)
    }
}
