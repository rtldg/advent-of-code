#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use core::cmp::Ordering;
use core::cmp::{max, min};
use std::{collections::HashMap, collections::HashSet, error::Error};

use itertools::Itertools;

fn main_part1() -> Result<(), Box<dyn Error>> {
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

    // println!("1 to 2 - {:?}", walk_from_a_to_b((4, 0), (9, 1)));
    // println!("5 to 9 - {:?}", walk_from_a_to_b((1, 6), (5, 11)));
    // println!("1 to 7 - {:?}", walk_from_a_to_b((4, 0), (10, 9)));
    // println!("3 to 6 - {:?}", walk_from_a_to_b((0, 2), (12, 7)));
    // println!("8 to 9 - {:?}", walk_from_a_to_b((0, 12), (5, 12)));

    let mut unordered_galaxies = vec![];

    for (y, line) in input0.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                unordered_galaxies.push((x, y));
            }
        }
    }

    let mut distance_sum = 0;
    for perm in (0..unordered_galaxies.len()).combinations(2) {
        distance_sum += walk_from_a_to_b(unordered_galaxies[perm[0]], unordered_galaxies[perm[1]]);
    }

    println!("\n\n{:?}\n\n", distance_sum);

    Ok(())
}

fn main_part2() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    let input_file = "input";
    let mut input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    let mut expanded_x = HashSet::new();
    let mut expanded_y = HashSet::new();

    // expand space vertically
    for i in (0..input0.len()).rev() {
        if input0[i].chars().unique().count() == 1 {
            expanded_y.insert(i);
        }
    }
    // expand space horizontally. did you know space is only 2d?
    'expand_hori: for x in (0..input0[0].len()).rev() {
        for y in 0..input0.len() {
            if input0[y].chars().nth(x).unwrap() != '.' {
                continue 'expand_hori;
            }
        }
        expanded_x.insert(x);
    }

    let walk_from_a_to_b_p2 = |(ax, ay), (mut bx, mut by), expanse_length: usize| -> usize {
        let (minx, mut maxx, miny, mut maxy) = (min(ax, bx), max(ax, bx), min(ay, by), max(ay, by));
        let x_expanses = expanded_x
            .iter()
            .map(|column| (minx..maxx + 1).contains(column))
            .filter(|v| *v)
            .count();
        let y_expanses = expanded_y
            .iter()
            .map(|row| (miny..maxy + 1).contains(row))
            .filter(|v| *v)
            .count();
        ((x_expanses + y_expanses) * (expanse_length - 1)) + maxx - minx + maxy - miny
    };

    // println!("1 to 2 - {:?}", walk_from_a_to_b((4, 0), (9, 1), 1));
    // println!("5 to 9 - {:?}", walk_from_a_to_b((1, 6), (5, 11), 1));
    // println!("1 to 7 - {:?}", walk_from_a_to_b((4, 0), (10, 9), 1));
    // println!("3 to 6 - {:?}", walk_from_a_to_b((0, 2), (12, 7), 1));
    // println!("8 to 9 - {:?}", walk_from_a_to_b((0, 12), (5, 12), 1));

    let mut unordered_galaxies = vec![];

    for (y, line) in input0.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                unordered_galaxies.push((x, y));
            }
        }
    }

    let (mut sums_p1, mut sums_p2) = (0, 0);
    for perm in (0..unordered_galaxies.len()).combinations(2) {
        let (a, b) = (unordered_galaxies[perm[0]], unordered_galaxies[perm[1]]);
        // sums_p1 += walk_from_a_to_b_p1(a, b);
        sums_p2 += walk_from_a_to_b_p2(a, b, 1_000_000);
    }

    println!("\n{}\n\n", sums_p2);

    Ok(())
}

fn main() {
    // I broke my part 1 when doing part 2 things so I don't want to debug it...
    main_part1().unwrap();
    main_part2().unwrap();
}
