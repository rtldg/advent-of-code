#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::cmp::Ordering;
use std::io::repeat;
use std::{collections::HashMap, collections::HashSet, error::Error};

use itertools::Itertools;
use rustc_hash::FxHashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    let input_file = "test_input_p2";
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
        let (a, b, c) = line
            .split_ascii_whitespace()
            .map(|s| {
                s.trim_start_matches('(')
                    .trim_end_matches(&[',', ')'])
                    .to_owned()
            })
            .collect_tuple()
            .unwrap();
        portal.insert(a, (b, c));
    }

    let mut steps_to_zzz = 0;
    let mut where_i_am = "AAA".to_owned();

    // while let Some(c) = repeated_directions.pop() { // would need to reverse string...
    loop {
        let c = repeated_directions.remove(0);
        steps_to_zzz += 1;

        // let ccc = where_i_am.clone();
        where_i_am = match c {
            'L' => portal.get(&where_i_am).unwrap().0.clone(),
            'R' => portal.get(&where_i_am).unwrap().1.clone(),
            _ => Err("huh")?,
        };
        // println!("{} | {}->{}", c, ccc, where_i_am);

        if where_i_am == "ZZZ" {
            break;
        }
    }

    println!("\n\n{:?}\n\n", steps_to_zzz);

    // part 2
    // part 2
    // part 2
    // part 2
    // part 2
    // part 2
    // part 2

    let mut input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(str::to_string)
        .map(|s| {
            s.replace("(", "")
                .replace(")", "")
                .replace(",", "")
                .replace("=", "")
        })
        .collect();

    let original_directions = input0.remove(0);
    let mut repeated_directions = original_directions.repeat(200000);

    // let mut portal = HashMap::new();
    let mut portal: FxHashMap<u32, (u32, u32)> = FxHashMap::default();

    for (lineid, line) in input0.iter().enumerate() {
        // let line = line
        //     .chars()
        //     .filter(|c| *c != ',' && *c != '(' && *c != ')' && *c != '=') // bleh
        //     .join("");
        let (a, b, c) = line
            .split_ascii_whitespace()
            .map(|s| s.as_bytes())
            .collect_tuple()
            .unwrap();
        let btou32 = |x: &[u8]| ((x[0] as u32) << 16) | ((x[1] as u32) << 8) | (x[2] as u32);
        portal.insert(btou32(a), (btou32(b), btou32(c)));
    }

    let mut steps_to_zzz: usize = 0;
    let mut where_i_am = vec![];
    for x in portal.iter() {
        if (x.0 & 0xFF) == (b'A' as u32) {
            where_i_am.push(*x.0);
        }
    }
    println!("{:?}", where_i_am.len());

    'mainloop: for c in original_directions.chars().cycle() {
        steps_to_zzz += 1;

        for x in where_i_am.iter_mut() {
            *x = if c == 'L' {
                portal.get(x).unwrap().0
            } else {
                portal.get(x).unwrap().1
            };
        }

        if steps_to_zzz % 1000000 == 0 {
            println!("{}  {:?}", steps_to_zzz, where_i_am);
        }

        for x in &where_i_am {
            if (*x & 0xFF) != (b'Z' as u32) {
                continue 'mainloop;
            }
        }

        break;
    }

    println!("\n\n{:?}\n\n", steps_to_zzz);

    Ok(())
}
