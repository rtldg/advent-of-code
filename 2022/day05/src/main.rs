fn hello(ispart2: bool) {
    let mut buckets: Vec<Vec<char>> = vec![Vec::new(); 9];

    for line in std::fs::read_to_string("input").unwrap().split('\n') {
        if line.contains("[") {
            for i in 0..9 {
                let c = line.as_bytes()[i * 4 + 1];
                if c != b' ' {
                    buckets[i].insert(0, c as char);
                }
            }
        } else if line.starts_with("move") {
            let cmd: Vec<usize> = line.split(' ').filter_map(|s| s.parse().ok()).collect();
            let len = buckets[cmd[1] - 1].len();
            let mut elems: Vec<char> = buckets[cmd[1] - 1].drain((len - cmd[0])..len).collect();
            if !ispart2 {
                elems.reverse();
            }
            buckets[cmd[2] - 1].extend(elems);
        }
    }

    for b in buckets {
        print!("{}", b.last().unwrap());
    }
    println!("");
}

fn main() {
    hello(false);
    hello(true);
}
