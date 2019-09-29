use std::fmt;
use itertools::Itertools;
use core::cmp;
use core::num::Wrapping as W;

// use
//fn undef() -> ! {
//    unimplemented!()
//}

pub trait Trim {
    fn trim_indent(self) -> String;
    fn replace_indent(self, new_indent: &str) -> String;
}

impl<'a> Trim for &'a str {
    fn trim_indent(self) -> String {
        replace_indent(self, &"")
    }
    fn replace_indent(self, new_indent: &str) -> String {
        replace_indent(self, new_indent)
    }
}

pub fn replace_indent<'a>(src: &'a str, new_indent: &'a str) -> String {
    let lines = src.lines().collect_vec();
    let min_common_indent = lines.iter()
        .filter(|s| !is_blank(&s))
        .map(|s| indent_width(s))
        .min()
        .unwrap_or(0);
    let f1: Box<dyn Fn(&str) -> String> = get_add_function(new_indent);
    let f2: Box<dyn Fn(&str) -> String> = get_cut_function(min_common_indent);
    reindent(&lines, src.len(), f1, f2)
}

pub fn get_add_function<'a>(indent: &'a str) -> Box<dyn Fn(&str) -> String + 'a> {
    if indent.is_empty() {
        Box::new(move |line: &str| { line.to_string() })
    } else {
        Box::new(move |line: &str| {
            let mut s = String::new();
            s.push_str(indent);
            s.push_str(line);
            s
        })
    }
}

pub fn get_cut_function(indent: usize) -> Box<dyn Fn(&str) -> String> {
    Box::new(move |line: &str| {
        // ensure all our values >= 0
        let n = cmp::max(1, line.len()) - 1;
        let idx = cmp::min(n, indent);
        line[idx..].to_string()
    })
}

pub fn reindent<F>(xs: &[&str], exp_size: usize, indent_add_f: Box<F>, indent_cut_f: Box<F>) -> String
    where
        F: for<'a> Fn(&'a str) -> String + ?Sized,
{
    let mut ys: Vec<String> = Vec::with_capacity(exp_size);
    for (i, x) in xs.iter().enumerate() {
        // exclude the first and the last line, skip blanks
        if i != 0 && i + 1 != xs.len() || !is_blank(x) {
            let x1 = indent_cut_f(x);
            let x2 = indent_add_f(x1.as_str());
            ys.push(x2);
        }
    }
    ys.join("\n")
}

pub fn indent_width(s: &str) -> usize {
    s.chars().position(|c| !c.is_ascii_whitespace()).unwrap_or(s.len())
}

pub fn is_blank(s: &str) -> bool {
    s.len() == 0 ||
        s.char_indices()
            .all(|(_, c)| c.is_ascii_whitespace())
}

// === Bound trait ===

pub trait Bound<T> {
    fn bound(self, lower: T, upper: T) -> T;
}

impl<T> Bound<T> for T where T: PartialOrd {
    fn bound(self, lower: T, upper: T) -> T {
        if self <= lower { lower }
            else if self >= upper { upper }
                else { self }
    }
}

// some tricks to un-hide IsaacRng contents

const RAND_SIZE_LEN: usize = 8;
const RAND_SIZE: usize = 1 << RAND_SIZE_LEN;

#[derive(Clone, Debug)]
pub struct IsaacRng0(pub BlockRng0<IsaacCore0>);

#[derive(Clone, Debug)]
pub struct BlockRng0<R: ?Sized> {
    pub results: u32,
    pub index: usize,
    pub core: R,
}

#[derive(Clone)]
pub struct IsaacCore0 {
    mem: [W<u32>; RAND_SIZE],
    a: W<u32>,
    b: W<u32>,
    c: W<u32>,
}

impl fmt::Debug for IsaacCore0 {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.debug_struct("IsaacCore0")
            .field("mem", &format_args!("{:?}", &self.mem[..]))
            .field("a", &format_args!("{}", self.a))
            .field("b", &format_args!("{}", self.b))
            .field("c", &format_args!("{}", self.c))
            .finish()
    }
}

//pub fn join<T, I: Iterator<Item = &str>>(xs: I, sep: &T) -> Vec<T> {
//    let size = xs.iter().fold(0, |acc, v| acc + v.borrow().len());
//    let mut result = Vec::with_capacity(size + xs.len());
//    let mut first = true;
//    for v in xs {
//        if first {
//            first = false
//        } else {
//            result.push(sep.clone())
//        }
//        result.extend_from_slice(v.borrow())
//    }
//    result
//}
