#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::{collections::HashMap, collections::HashSet, error::Error};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    let input_file = "input";
    let input0 = std::fs::read_to_string(input_file)?;

    let mut input0: Vec<String> = std::fs::read_to_string(input_file)?
        // .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    let seeds = input0
        .remove(0)
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let (mut current_location, mut next_location) = (seeds.clone(), seeds.clone());

    for (lineid, line) in input0.iter().enumerate() {
        if line == "" || line.contains(" map:") {
            current_location = next_location.clone();
            continue;
        }

        let (destination_range_start, source_range_start, range_length) = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        for (cur, next) in current_location.iter_mut().zip(next_location.iter_mut()) {
            if *cur >= source_range_start && *cur <= source_range_start + range_length - 1 {
                println!("{} -- {}", cur, line);
                *next = *cur - source_range_start + destination_range_start;
            }
        }
    }

    println!("{:?}", current_location);

    println!("\n{}\n{}", current_location.iter().min().unwrap(), 0);
    Ok(())
}
