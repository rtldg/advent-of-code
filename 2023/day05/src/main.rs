#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::{collections::HashMap, collections::HashSet, error::Error, ops::Range};

use itertools::Itertools;
use rayon::prelude::*;

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
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();

    let (mut current_location, mut next_location) = (seeds.clone(), seeds.clone());

    for (lineid, line) in input0.iter().enumerate() {
        if line == "" || line.contains(" map:") {
            current_location = next_location.clone();
            continue;
        }

        let (destination_range_start, source_range_start, range_length) = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();

        for (cur, next) in current_location.iter_mut().zip(next_location.iter_mut()) {
            if *cur >= source_range_start && *cur <= source_range_start + range_length - 1 {
                // println!("{} -- {}", cur, line);
                *next = *cur - source_range_start + destination_range_start;
            }
        }
    }

    println!(
        "{:?}\n\n{:?}\n\n",
        current_location,
        current_location.iter().min().unwrap()
    );

    // PART 2 SHIT

    let mut part2_map = vec![];

    for (lineid, line) in input0.iter().enumerate() {
        if line == "" || line.contains(" map:") {
            part2_map.push((-1, 0..1));
            continue;
        }

        let (destination_range_start, source_range_start, range_length) = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();

        part2_map.push((
            destination_range_start,
            source_range_start..source_range_start + range_length,
        ));
    }

    let get_location = |seed: isize, part2_map: &[(isize, Range<isize>)]| {
        let mut current_location = seed;
        let mut skip = false;

        for location in part2_map {
            if location.0 == -1 {
                skip = false;
                continue;
            } else if skip {
                continue;
            } else if location.1.contains(&current_location) {
                skip = true;
                current_location = current_location - location.1.start + location.0;
            }
        }

        current_location
    };

    // let mut seeds_to_check = 0;
    // for i in 0..seeds.len() / 2 {
    //     seeds_to_check += seeds[i * 2 + 1];
    // }

    let mut min_location = 9999999999999999isize;
    let mut iters = 0;
    for i in 0..seeds.len() / 2 {
        let (smin, smax) = (seeds[i * 2], seeds[i * 2] + seeds[i * 2 + 1]);

        min_location = core::cmp::min(
            min_location,
            (smin..smax)
                .into_par_iter()
                .map(|seed| get_location(seed, &part2_map))
                .min()
                .unwrap(),
        );
        println!("{}/{} -- {}", i + 1, seeds.len() / 2, min_location);
        // for seed in smin..smax {
        //     iters += 1;
        //     if iters > 0 && iters % 200000 == 0 {
        //         println!(
        //             "100k iters -- {}% -- {}",
        //             iters * 100 / seeds_to_check,
        //             min_location
        //         );
        //     }
        //     min_location = core::cmp::min(min_location, get_location(seed, &part2_map));
        // }
    }

    println!("\n{}", min_location);
    Ok(())
}
