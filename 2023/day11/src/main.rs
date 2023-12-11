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
    // let input_file = "test_input_p2";
    // let input_file = "input";
    let mut input0 = std::fs::read_to_string(input_file)?;
    let mut input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    for (lineid, line) in input0.iter().enumerate() {

    }

    println!("\n\n{:?}\n\n", 0);


    Ok(())
}
