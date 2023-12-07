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

    let mut input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    let determine_type = |hand: &str| -> usize {
        let mut chars = HashMap::new();
        for c in hand.chars() {
            chars.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut chars = chars.iter().map(|x| (*x.0, *x.1)).collect_vec();
        chars.sort_by(|a, b| b.1.cmp(&a.1));

        if chars[0].1 == 5 {
            return 7;
        } else if chars[0].1 == 4 {
            return 6;
        } else if chars[0].1 == 3 && chars[1].1 == 2 {
            return 5;
        } else if chars[0].1 == 3 && chars[1].1 == 1 {
            return 4;
        } else if chars[0].1 == 2 && chars[1].1 == 2 {
            return 3;
        } else if chars[0].1 == 2 && chars[1].1 == 1 {
            return 2;
        } else {
            return 1;
        }
    };

    let mut sets: Vec<(&str, &str)> = input0
        .iter()
        .map(|s| s.split(' ').collect_tuple().unwrap())
        .collect_vec();
    sets.sort_by(|(ahand, _), (bhand, _)| {
        let (atype, btype) = (determine_type(&ahand), determine_type(&bhand));
        if atype > btype {
            Ordering::Greater
        } else if atype < btype {
            Ordering::Less {}
        } else {
            for (ac, bc) in ahand.chars().zip(bhand.chars()) {
                let strength = "23456789TJQKA";
                let strength_comparison =
                    strength.find(ac).unwrap().cmp(&strength.find(bc).unwrap());
                if strength_comparison != Ordering::Equal {
                    return strength_comparison;
                }
            }
            Ordering::Equal
        }
    });

    // println!("{:?}", sets);

    let total_winnings: usize = sets
        .iter()
        .enumerate()
        .map(|(i, set)| (i + 1) * set.1.parse::<usize>().unwrap())
        .sum();

    println!("\n\n{:?}\n\n", total_winnings);

    // part 2
    // part 2
    // part 2
    // part 2

    let determine_type = |hand: &str| -> usize {
        let mut chars_map = HashMap::new();
        for c in hand.chars() {
            chars_map.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut chars = chars_map.iter().map(|x| (*x.0, *x.1)).collect_vec();
        chars.sort_by(|a, b| b.1.cmp(&a.1));

        if let Some(number_of_j) = chars_map.get(&'J') && *number_of_j != 5 {
            chars.retain(|x| x.0 != 'J');
            chars[0].1 += number_of_j;
            chars.sort_by(|a, b| b.1.cmp(&a.1));
        }

        if chars[0].1 == 5 {
            return 7;
        } else if chars[0].1 == 4 {
            return 6;
        } else if chars[0].1 == 3 && chars[1].1 == 2 {
            return 5;
        } else if chars[0].1 == 3 && chars[1].1 == 1 {
            return 4;
        } else if chars[0].1 == 2 && chars[1].1 == 2 {
            return 3;
        } else if chars[0].1 == 2 && chars[1].1 == 1 {
            return 2;
        } else {
            return 1;
        }
    };

    let mut sets: Vec<(&str, &str)> = input0
        .iter()
        .map(|s| s.split(' ').collect_tuple().unwrap())
        .collect_vec();
    sets.sort_by(|(ahand, _), (bhand, _)| {
        let (atype, btype) = (determine_type(&ahand), determine_type(&bhand));
        if atype > btype {
            Ordering::Greater
        } else if atype < btype {
            Ordering::Less {}
        } else {
            for (ac, bc) in ahand.chars().zip(bhand.chars()) {
                let strength = "J23456789TQKA";
                let strength_comparison =
                    strength.find(ac).unwrap().cmp(&strength.find(bc).unwrap());
                if strength_comparison != Ordering::Equal {
                    return strength_comparison;
                }
            }
            Ordering::Equal
        }
    });

    // println!("{:?}", sets);

    let total_winnings: usize = sets
        .iter()
        .enumerate()
        .map(|(i, set)| (i + 1) * set.1.parse::<usize>().unwrap())
        .sum();

    println!("\n\n{:?}\n\n", total_winnings);
    Ok(())
}
