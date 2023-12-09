#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::cmp::Ordering;
use std::{collections::HashMap, collections::HashSet, error::Error};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    let input_file = "input";
    let input0 = std::fs::read_to_string(input_file)?;
    let mut input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    // let mut things: Vec<Vec<i32>> = vec![];
    let mut extrapolated_values = vec![];
    for line in input0.iter() {
        let mut history_idx = 0;
        let mut history: Vec<Vec<i32>> = vec![line
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect_vec()];

        while history[history.len()-1].iter().find(|x| **x != 0).is_some() {
            let mut current_row = vec![];
            for i in 1..history[history.len()-1].len() {
                current_row.push(history[history.len()-1][i] - history[history.len()-1][i-1]);
            }
            history.push(current_row);
            // println!("{:?}", current_row);
            //
        }

        for row in (1..history.len()).rev() {
            let column = history[row].len()-1;
            let extrapolated_value = history[row-1][column] + history[row][column];
            extrapolated_values.push(extrapolated_value);
            history[row].push(extrapolated_value);
        }

        // let mut extrapolated_value = 0;
        // for row in history.iter() {
        //     extrapolated_value += row[row.len()-1];
        // }
        // extrapolated_values.push(extrapolated_value);
    }

    // println!("{:?}", things);

    println!("\n\n{:?}\n\n", extrapolated_values.iter().sum::<i32>());
    Ok(())
}
