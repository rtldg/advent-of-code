use std::collections::HashSet;

fn hello(count: usize) {
    let s = std::fs::read_to_string("input").unwrap();

    for i in count..s.len() {
        let current = &s[i - count..i];
        let mut set: HashSet<char> = HashSet::new();
        for x in current.chars() {
            set.insert(x);
        }
        if set.len() == count {
            println!("{}", i);
            break;
        }
        // println!("{}", &s[i-4..i]);
    }
}

fn main() {
    hello(4);
    hello(14);
}
