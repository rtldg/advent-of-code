#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use core::cmp::Ordering;
use core::cmp::{max, min};
use std::{collections::HashMap, collections::HashSet, error::Error};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    // let input_file = "test_input_p2";
    // let input_file = "input";
    let mut input0 = std::fs::read_to_string(input_file)?;
    let mut input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    let mut arrangements = 0;
    for (lineid, line) in input0.iter().enumerate() {
        let (condition, contiguous_damaged) = line.split_ascii_whitespace().collect_tuple().unwrap();
        let contiguous_damaged = contiguous_damaged
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();
        let relevant = condition.split('.').filter(|s| s.len() > 0).collect_vec();
        println!("{:?}", relevant);

    }

    println!("\n\n{:?}\n\n", arrangements);

    Ok(())
}
