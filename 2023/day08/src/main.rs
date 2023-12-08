#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::cmp::Ordering;
use std::io::repeat;
use std::{collections::HashMap, collections::HashSet, error::Error};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    let input_file = "input";
    let input0 = std::fs::read_to_string(input_file)?;

    let mut input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(str::to_string)
        .collect();

    let original_directions = input0.remove(0);
    let mut repeated_directions = original_directions.repeat(2000);

    let mut portal = HashMap::new();

    for (lineid, line) in input0.iter().enumerate() {
        let line = line
            .chars()
            .filter(|c| *c != ',' && *c != '(' && *c != ')' && *c != '=') // bleh
            .join("");
        let (a, b, c) = line.split_ascii_whitespace().collect_tuple().unwrap();
        portal.insert(a.to_owned(), (b.to_owned(), c.to_owned()));
    }

    let mut steps_to_zzz = 0;
    let mut where_i_am = "AAA".to_owned();

    // while let Some(c) = repeated_directions.pop() { // would need to reverse string...
    loop {
        let c = repeated_directions.remove(0);
        steps_to_zzz += 1;

        // let ccc = where_i_am.clone();
        where_i_am = match c {
            'L' => {
                portal.get(&where_i_am).unwrap().0.clone()
            },
            'R' => {
                portal.get(&where_i_am).unwrap().1.clone()
            },
            _ => Err("huh")?
        };
        // println!("{} | {}->{}", c, ccc, where_i_am);


        if where_i_am == "ZZZ" {
            break;
        }
    }

    println!("\n\n{:?}\n\n", steps_to_zzz);
    Ok(())
}
