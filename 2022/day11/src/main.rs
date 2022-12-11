use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey {
    inspects: usize,
    items: VecDeque<usize>,
    ops: Vec<String>,
    divisor: usize,
    ontrue: usize,
    onfalse: usize,
}

fn hello(ispart2: bool) {
    let input = std::fs::read_to_string("input").unwrap();
    let mut input: VecDeque<&str> = input.trim().split('\n').collect();
    let mut monkeys: Vec<Monkey> = vec![];

    for _ in 0..input.len() / 7 + 1 {
        let _ = input.pop_front(); // bye bye "Monkey X:"
        monkeys.push(Monkey {
            inspects: 0,
            items: input.pop_front().unwrap().split(":").collect::<Vec<&str>>()[1]
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect(),
            ops: input
                .pop_front()
                .unwrap()
                .split(' ')
                .skip(6)
                .map(|s| s.to_owned())
                .collect(),
            divisor: input.pop_front().unwrap().split(' ').collect::<Vec<&str>>()[5]
                .trim()
                .parse()
                .unwrap(),
            ontrue: input
                .pop_front()
                .unwrap()
                .split(' ')
                .skip(9)
                .next()
                .unwrap()
                .parse()
                .unwrap(),
            onfalse: input
                .pop_front()
                .unwrap()
                .split(' ')
                .skip(9)
                .next()
                .unwrap()
                .parse()
                .unwrap(),
        });
        let _ = input.pop_front(); // empty line

        // println!("{:?}", monkeys[monkeys.len()-1]);
    }

    let lcm = monkeys.iter().fold(1, |acc, x| acc * x.divisor);

    let rounds = if ispart2 { 10000 } else { 20 };
    for _round in 0..rounds {
        // println!("round {}", _round);
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys[m].items.pop_front() {
                monkeys[m].inspects += 1;
                let other = monkeys[m].ops[1].as_str().parse().unwrap_or(item);
                let item = match monkeys[m].ops[0].as_str() {
                    "*" => item * other,
                    "+" => item + other,
                    _ => panic!("hi"),
                };
                let item = if ispart2 { item } else { item / 3 };
                let receiver = if (item % monkeys[m].divisor) == 0 {
                    monkeys[m].ontrue
                } else {
                    monkeys[m].onfalse
                };
                monkeys[receiver].items.push_back(item % lcm);
            }
        }
    }

    monkeys
        .as_mut_slice()
        .sort_by(|a, b| a.inspects.cmp(&b.inspects));
    monkeys.as_mut_slice().reverse();

    // println!("{:?}", monkeys);
    println!("{}", monkeys[0].inspects * monkeys[1].inspects);
}

fn main() {
    hello(false);
    hello(true);
}
