#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{error::Error, collections::HashSet};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    let input_file = "input";
    let input0 = std::fs::read_to_string(input_file)?;

    let input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    let mut total_point_worth = 0;

    for (lineid, line) in input0.iter().enumerate() {
        let (winning, mine) = line.split(':').nth(1).unwrap().split('|').collect_tuple().unwrap();
        let winning: HashSet<&str> = HashSet::from_iter(winning.split_ascii_whitespace());
        let mine: HashSet<&str> = HashSet::from_iter(mine.split_ascii_whitespace());
        /*let mut winning_hashset = HashSet::new();
        let _ = winning.split_ascii_whitespace().map(|n| winning_hashset.insert(n.parse::<usize>())).collect();
        for _ =*/
        let matches = winning.intersection(&mine).count();
        println!("{}", matches);

        if matches > 0 {
            total_point_worth += 1 << (matches-1);
        }
    }

    println!("\n{}\n{}", total_point_worth, 0);
    Ok(())
}
