use std::collections::HashSet;

fn hello(count: usize) {
    let s = std::fs::read_to_string("input").unwrap();

    for i in count..s.len() {
        let current = &s[i - count..i];
        if HashSet::<char>::from_iter(current.chars()).len() == count {
            println!("{}", i);
            return;
        }
    }
}

fn main() {
    hello(4);
    hello(14);
}
