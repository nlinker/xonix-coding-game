extern crate itertools;

use std::iter::Iterator;
use self::itertools::Itertools;
use std::slice::Iter;

pub fn trim_indent(src: &str) -> &str {
    replace_indent(&src, &"")
}

pub fn replace_indent<'a>(src: &'a str, new_indent: &'a str) -> &'a str {
    let lines = src.lines().collect_vec();
    let min_common_indent = lines.iter()
        .filter(|s| !is_blank(&s))
        .map(|s| indent_width(s))
        .min()
        .unwrap_or(0);
    println!("min_common_indent = {:?}", min_common_indent);
    let exp_size = src.len() + new_indent.len() * lines.len();
    //let _: Iter<&str> = lines.iter();
    let f1 = get_add_function(&new_indent);
    let f2 = get_cut_function(min_common_indent);
    reindent(lines.iter(), exp_size, f1, f2);
    src
}

pub fn get_add_function<'a>(indent: &'a str) -> Box<Fn(&str) -> String + 'a> {
    Box::new(move |line: &str| format!("{}{}", indent, line))
}

pub fn get_cut_function(min_common_indent: usize) -> Box<Fn(&str) -> String> {
    Box::new(move |line: &str| { line[min_common_indent..].to_string() })
}

//pub fn reindent<'a, F1, F2>(xs: Iter<&str>,
//                            exp_size: usize,
//                            indent_add_f: Box<F1>,
//                            indent_cut_f: Box<F2>) -> String
//    where
//        F1: Fn(&'a str) -> String,
//        F1: Fn(&'a str) -> String,
//{
//    unimplemented!()
////private inline fun List<String>.reindent(resultSizeEstimate: Int,
////                                         indentAddFunction: (String) -> String,
////                                         indentCutFunction: (String) -> String?): String {
////    val lastIndex = lastIndex
////    return mapIndexedNotNull { index, value ->
////            if ((index == 0 || index == lastIndex) && value.isBlank())
////                null
////            else
////                indentCutFunction(value)?.let(indentAddFunction) ?: value
////        }
////        .joinTo(StringBuilder(resultSizeEstimate), "\n")
////        .toString()
////}
//}

pub fn indent_width(s: &str) -> usize {
    s.chars().position(|c| !c.is_ascii_whitespace()).unwrap_or(s.len())
}

pub fn is_blank(s: &str) -> bool {
    s.len() == 0 ||
        s.char_indices()
            .all(|(_, c)| c.is_ascii_whitespace())
}
