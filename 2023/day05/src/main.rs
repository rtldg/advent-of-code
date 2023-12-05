#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::{collections::HashSet, error::Error};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    //let input_file = "input";
    let input0 = std::fs::read_to_string(input_file)?;

    let input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    for (lineid, line) in input0.iter().enumerate() {

    }

    println!("\n{}\n{}", 0, 0);
    Ok(())
}
