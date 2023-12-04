#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{collections::HashSet, error::Error};

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

    let mut card_counts = vec![1; input0.len()];

    for (cardid, line) in input0.iter().enumerate() {
        let (winning, mine) = line
            .split(':')
            .nth(1)
            .unwrap()
            .split('|')
            .collect_tuple()
            .unwrap();
        let winning: HashSet<&str> = HashSet::from_iter(winning.split_ascii_whitespace());
        let mine: HashSet<&str> = HashSet::from_iter(mine.split_ascii_whitespace());

        let matches = winning.intersection(&mine).count();
        // println!("{}", matches);

        if matches > 0 {
            total_point_worth += 1 << (matches - 1);

            for i in 1..matches + 1 {
                card_counts[cardid + i] += card_counts[cardid];
            }
        }
    }

    println!(
        "\n{}\n{}",
        total_point_worth,
        card_counts.iter().sum::<usize>()
    );
    Ok(())
}
