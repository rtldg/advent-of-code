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

	'lineloop: for (lineid, line) in input0.iter().enumerate() {
		// Alternatively we could check if the levels == sorted(levels) || levels == reverse(sorted(levels))

		let mut diff_sign: i32 = 0;
		let mut prev_num: i32 = 0;
		for thing in line.split_ascii_whitespace() {
			let num: i32 = thing.parse()?;
			if prev_num != 0 {
				let abs_diff = prev_num.abs_diff(num);
				if abs_diff < 1 || abs_diff > 3 {
					println!("skipping line {} - abs_diff = {abs_diff}", lineid + 1);
					continue 'lineloop;
				}

				let current_diff_sign = (prev_num - num).signum();

				if diff_sign == 0 {
					diff_sign = current_diff_sign;
				} else if diff_sign != current_diff_sign {
					println!(
						"skipping line {} - diff_sign = {diff_sign} & current_diff_sign = {current_diff_sign}",
						lineid + 1
					);
					continue 'lineloop;
				}
			}

			prev_num = num;
		}
		safe_reports += 1;
	}

	println!("\n\n{}\n\n", safe_reports);

	Ok(())
}
