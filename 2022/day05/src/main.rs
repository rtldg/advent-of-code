#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn hello(ispart2: bool) {
    let mut s = std::fs::read_to_string("input").unwrap();

    let mut buckets: Vec<Vec<char>> = Vec::with_capacity(9);
    for i in 0..9 {
        buckets.push(Vec::new());
    }
    let mut commands: Vec<(usize, usize, usize)> = Vec::new();
    let mut reading_boxes = true;

    for line in s.split('\n') {
        if reading_boxes && line.as_bytes()[0] == b' ' && line.as_bytes()[1] == b'1' {
            reading_boxes = false;
            continue;
        }
        if line == "" {
            continue;
        };
        if reading_boxes {
            for i in 0..9 {
                let c = line.as_bytes()[i * 4 + 1];
                if c != b' ' {
                    buckets[i].insert(0, c as char);
                };
            }
        } else {
            let mut cmd: Vec<&str> = line.split(' ').skip(1).collect();
            // println!("{:?}", cmd);
            commands.push((
                cmd[0].parse().unwrap(),
                cmd[2].parse().unwrap(),
                cmd[4].parse().unwrap(),
            ));
        }
    }

    // println!("{:?}", commands);
    for cmd in &commands {
        // println!("{:?}", buckets);
        if ispart2 {
            let len = buckets[cmd.1 - 1].len();
            let elems: Vec<char> = buckets[cmd.1 - 1].drain((len - cmd.0)..len).collect();
            buckets[cmd.2 - 1].extend(elems);
        } else {
            for i in 0..cmd.0 {
                let elem = buckets[cmd.1 - 1].pop().unwrap();
                buckets[cmd.2 - 1].push(elem);
            }
        }
    }

    for b in buckets {
        print!("{}", b[b.len() - 1]);
    }
    println!("");
}

fn main() {
    hello(false);
    hello(true);
}
