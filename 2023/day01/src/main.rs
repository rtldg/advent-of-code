#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    let input0 = std::fs::read_to_string("input").unwrap();

    let input0: Vec<String> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    let mut numbers1 = vec![];
    let mut numbers2 = vec![];

    let numberstrings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for line in &input0 {
        if line == "" {
            continue;
        }
        let mut first_pos = 9999999999;
        let mut first_val = 0;
        let mut last_pos = 0;
        let mut last_val = 0;
        for (i,c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if first_val == 0 {
                    first_pos = i;
                    first_val = c.to_digit(10).unwrap() as usize;
                }
                last_pos = i;
                last_val = c.to_digit(10).unwrap() as usize;
            }
        }
        numbers1.push(first_val*10 + last_val);
        for t in numberstrings {
            if let Some(x) = line.find(t) {
                if x <= first_pos {
                    first_pos = x;
                    first_val = numberstrings.iter().position(|&v| v == t).unwrap() + 1;
                }
            }
            if let Some(x) = line.rfind(t) {
                if x >= last_pos {
                    last_pos = x;
                    last_val = numberstrings.iter().position(|&v| v == t).unwrap() + 1;
                }
            }
        }
        numbers2.push(first_val*10 + last_val);
    }

    println!("{}\n{}", numbers1.iter().sum::<usize>(), numbers2.iter().sum::<usize>());
}
