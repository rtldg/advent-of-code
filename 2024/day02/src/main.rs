#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use core::cmp::Ordering;
use core::cmp::{max, min};
use std::{collections::HashMap, collections::HashSet, error::Error};

use anyhow::Context;
use itertools::{Itertools, rev, sorted};

fn main() -> anyhow::Result<()> {
	let input_file = "test_input";
	let input_file = "input";

	let mut input0: Vec<String> = std::fs::read_to_string(input_file)
		.context("failed to read input_file to string 2")?
		.trim()
		.split('\n')
		.map(str::to_string)
		.collect();

	let mut safe_reports = 0;
	let mut safe_and_dampened_reports = 0;

	'lineloop: for (lineid, line) in input0.iter().enumerate() {
		let original_levels: Vec<i32> = line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();

		// Check if levels == sorted(levels).
		// Check if levels == rev(sorted(levels)).
		// Check if abs_diff is correct.

		'levelloop: for remove_idx in (0..=original_levels.len()).rev() {
			let mut levels = original_levels.clone();
			if remove_idx < original_levels.len() {
				levels.remove(remove_idx);
			}

			let mut sorted = levels.clone();
			sorted.sort();
			let mut rev_sorted = sorted.clone();
			rev_sorted.reverse();

			if levels == sorted || levels == rev_sorted {
				for win in levels.windows(2) {
					let abs_diff = win[0].abs_diff(win[1]);
					if abs_diff < 1 || abs_diff > 3 {
						continue 'levelloop;
					}
				}

				safe_and_dampened_reports += 1;
				if remove_idx == original_levels.len() {
					safe_reports += 1;
				}

				continue 'lineloop;
			}
		}
	}

	println!("\n\n{}\n\n{}\n\n", safe_reports, safe_and_dampened_reports);

	Ok(())
}
