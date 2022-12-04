#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn part2() {
    let mut answer: u32 = 0;
    let mut s = std::fs::read_to_string("input").unwrap();
    for line in s.split('\n') {
        let pairs: Vec<Vec<i32>> = line
            .split(',')
            .map(|z| z.split('-').map(|y| y.parse::<i32>().unwrap()).collect())
            .collect();

        if pairs[0][0] <= pairs[1][1] && pairs[0][1] >= pairs[1][0] {
            answer += 1;
        }
    }
    println!("{}", answer);
}

fn part1() {
    let mut answer: u32 = 0;
    let mut s = std::fs::read_to_string("input").unwrap();
    for line in s.split('\n') {
        let pairs: Vec<Vec<i32>> = line
            .split(',')
            .map(|z| z.split('-').map(|y| y.parse::<i32>().unwrap()).collect())
            .collect();
        if pairs[0][0] <= pairs[1][0] && pairs[0][1] >= pairs[1][1] {
            // println!("{:?}", pairs);
            answer += 1;
        } else if pairs[1][0] <= pairs[0][0] && pairs[1][1] >= pairs[0][1] {
            // println!("{:?}", pairs);
            answer += 1;
        }
    }
    println!("{}", answer);
}

fn main() {
    part1();
    part2();
}
