#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in std::fs::read_to_string("input").unwrap().split('\n') {
        let pairs: Vec<Vec<i32>> = line
            .split(',')
            .map(|z| z.split('-').map(|y| y.parse::<i32>().unwrap()).collect())
            .collect();

        if pairs[0][0] <= pairs[1][0] && pairs[0][1] >= pairs[1][1] {
            p1 += 1;
        } else if pairs[1][0] <= pairs[0][0] && pairs[1][1] >= pairs[0][1] {
            p1 += 1;
        }

        if pairs[0][0] <= pairs[1][1] && pairs[0][1] >= pairs[1][0] {
            p2 += 1;
        }
    }

    println!("{}\n{}", p1, p2);
}
