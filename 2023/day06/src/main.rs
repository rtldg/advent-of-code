#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::{collections::HashMap, collections::HashSet, error::Error};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    let input_file = "input";

    let mut input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();
    let times = input0[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    let distances = input0[1]
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let mut win_counts = vec![0; times.len()];
    for (time_limit, target_distance, win_count) in
        itertools::izip!(times, distances, win_counts.iter_mut())
    {
        for i in 1..time_limit {
            let x = (time_limit - i) * i;
            if x > target_distance {
                *win_count += 1;
            }
        }
    }

    println!("\n\n{}", win_counts.iter().product::<usize>());

    // PART 2

    let time_limit = input0[0]
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse::<usize>()
        .unwrap();
    let target_distance = input0[1]
        .split_ascii_whitespace()
        .skip(1)
        .join("")
        .parse::<usize>()
        .unwrap();

    let mut win_count = 0;
    for i in 1..time_limit {
        let x = (time_limit - i) * i;
        if x > target_distance {
            win_count += 1;
        }
    }

    println!("\n\n{}", win_count);
    Ok(())
}
