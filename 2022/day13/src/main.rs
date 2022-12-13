use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Thing {
    List(Vec<Thing>),
    Atom(u8),
}
impl Ord for Thing {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_lists(self, other)
    }
}
impl PartialOrd for Thing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Thing {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Thing {}

fn get_thing(i: &mut usize, l: &[u8]) -> Thing {
    let mut things = vec![];
    while *i < l.len() {
        let c = l[*i];
        *i += 1;
        if c == b',' {
            continue;
        }
        if c == b']' {
            break;
        } else if c == b'[' {
            things.push(get_thing(i, l));
        } else {
            let mut n = c - b'0';
            while let Some(c) = (l[*i] as char).to_digit(10) {
                n = n * 10 + c as u8;
                *i += 1;
            }
            things.push(Thing::List(vec![Thing::Atom(n as u8)]));
        }
    }
    return Thing::List(things);
}

fn compare_lists(l: &Thing, r: &Thing) -> Ordering {
    let (mut a, mut b) = (vec![], vec![]);
    let l = match l {
        Thing::List(x) => x,
        Thing::Atom(x) => {
            a.push(Thing::Atom(*x));
            &a
        }
    };
    let r = match r {
        Thing::List(x) => x,
        Thing::Atom(x) => {
            b.push(Thing::Atom(*x));
            &b
        }
    };
    for (i, thing) in l.iter().enumerate() {
        if i + 1 > r.len() {
            return Ordering::Greater;
        }
        // println!("{}\n{:?}\n{:?}", i, thing, r[i]);
        match thing {
            Thing::List(_) => match &r[i] {
                Thing::Atom(_) => {
                    let order = compare_lists(&thing, &Thing::List(vec![r[i].clone()]));
                    if order != Ordering::Equal {
                        return order;
                    }
                }
                Thing::List(_) => {
                    let order = compare_lists(thing, &r[i]);
                    if order != Ordering::Equal {
                        return order;
                    }
                }
            },
            Thing::Atom(x) => match &r[i] {
                Thing::Atom(y) => {
                    if x != y {
                        return x.cmp(y);
                    }
                }
                Thing::List(_) => {
                    let order = compare_lists(&Thing::List(vec![thing.clone()]), &r[i]);
                    if order != Ordering::Equal {
                        return order;
                    }
                }
            },
        }
    }
    if l.len() < r.len() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn hello() {
    let input: Vec<String> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .split("\n\n")
        .map(str::to_string)
        .collect();
    let mut answer: usize = 0;

    for (i, pair) in input.iter().enumerate() {
        if i + 1 != 4 {
            // continue;
        }
        let pair: Vec<&[u8]> = pair.split('\n').map(|x| x.as_bytes()).collect();
        // println!("{:?}", pair);
        let l = get_thing(&mut 0, pair[0]);
        let r = get_thing(&mut 0, pair[1]);
        if l < r {
            //println!("Good {}", i + 1);
            answer += i + 1;
        } else {
            // println!("Bad {} -- {:?}", i + 1, pair);
        }
        // println!("{:?}\n{:?}", l, r);
    }

    println!("\n{}", answer);
}

fn part2() {
    let input: Vec<String> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();
    let mut packets: Vec<Thing> = vec![];
    let div1 = get_thing(&mut 0, b"[[2]]");
    let div2 = get_thing(&mut 0, b"[[6]]");
    packets.push(div1.clone());
    packets.push(div2.clone());

    for line in input.iter() {
        let line = line.trim();
        if line != "" {
            packets.push(get_thing(&mut 0, line.as_bytes()));
        }
    }

    packets.sort();
    let div1idx = packets.iter().position(|x| x == &div1).unwrap() + 1;
    let div2idx = packets.iter().position(|x| x == &div2).unwrap() + 1;
    println!("{} * {} = {}", div1idx, div2idx, div1idx * div2idx);
}

fn main() {
    hello();
    part2();
}
