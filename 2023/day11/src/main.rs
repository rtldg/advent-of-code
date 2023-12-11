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
    let input_file = "input";
    let mut input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    // expand space vertically
    for i in (0..input0.len()).rev() {
        if input0[i].chars().unique().count() == 1 {
            input0.insert(i, input0[i].clone());
        }
    }
    // expand space horizontally. did you know space is only 2d?
    'expand_hori: for x in (0..input0[0].len()).rev() {
        for y in 0..input0.len() {
            if input0[y].chars().nth(x).unwrap() != '.' {
                continue 'expand_hori;
            }
        }
        for y in 0..input0.len() {
            input0[y].insert(x, '.');
        }
    }

    let walk_from_a_to_b = |(ax, ay): (usize, usize), (bx, by): (usize, usize)| -> usize {
        let mut steps = 0;
        let (mut dx, mut dy) = (ax as isize - bx as isize, ay as isize - by as isize);
        while dx != 0 || dy != 0 {
            if dx != 0 {
                dx -= dx.signum();
                steps += 1;
            }
            if dy != 0 {
                dy -= dy.signum();
                steps += 1;
            }
        }
        steps
    };

    println!("1 to 2 - {:?}", walk_from_a_to_b((4, 0), (9, 1)));
    println!("5 to 9 - {:?}", walk_from_a_to_b((1, 6), (5, 11)));
    println!("1 to 7 - {:?}", walk_from_a_to_b((4, 0), (10, 9)));
    println!("3 to 6 - {:?}", walk_from_a_to_b((0, 2), (12, 7)));
    println!("8 to 9 - {:?}", walk_from_a_to_b((0, 12), (5, 12)));

    let mut unordered_galaxies = vec![];

    for (y, line) in input0.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                unordered_galaxies.push((x, y));
            }
        }
    }

    /*
    let mut visited = HashSet::new();
    visited.insert(unordered_galaxies[0]);
    let mut current_galaxy = unordered_galaxies[0];

    let mut galaxy_distances = vec![];
    while visited.len() != unordered_galaxies.len() {
        println!("{} - {}", visited.len(), unordered_galaxies.len());
        let mut closest_galaxy = (999999, (999999,999999));
        for galaxy in unordered_galaxies.iter() {
            if !visited.contains(galaxy) {
                let steps = walk_from_a_to_b(current_galaxy, *galaxy);
                if steps < closest_galaxy.0 {
                    closest_galaxy = (steps, *galaxy);
                }
            }
        }
        galaxy_distances.push(closest_galaxy.0);
        visited.insert(closest_galaxy.1);
        current_galaxy = closest_galaxy.1;
    }
    println!("\n\n{:?}\n\n", galaxy_distances.iter().sum::<usize>());
    */

    let mut distance_sum = 0;
    let mut c = 0;
    for perm in (0..unordered_galaxies.len()).combinations(2) {
        c += 1;
        distance_sum += walk_from_a_to_b(unordered_galaxies[perm[0]], unordered_galaxies[perm[1]]);
    }

    println!("\n\n{} - {:?}\n\n", c, distance_sum);

    Ok(())
}
