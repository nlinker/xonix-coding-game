trait Tx {
    fn do_tx(&mut self);
}

struct S<'a>(&'a str);

impl<'a> Tx for S<'a> {
    fn do_tx(&mut self) {
        // technically we want to modify something
        println!("do_tx: {}", self.0)
    }
}

fn do_all(boxes: &mut [Box<dyn Tx>]) {
    let n = boxes.len();
    for k in 0..n {
        boxes[k].do_tx();
    }
}

fn main() {
    let mut boxes: [Box<dyn Tx>; 3] = [Box::new(S("a")), Box::new(S("b")), Box::new(S("c"))];
    do_all(&mut boxes);
}
