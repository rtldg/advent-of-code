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

    let (mut answer1, mut answer2) = (0, 0);

    println!("{}\n{}", answer1, answer2);
}
