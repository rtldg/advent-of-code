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

	let mut input0 = std::fs::read_to_string(input_file).context("failed to read input_file to string 1")?;

	let mul_re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

	let mut answerp1 = 0;

	for (_, [l, r]) in mul_re.captures_iter(&input0).map(|c| c.extract()) {
		answerp1 += l.parse::<i32>()? * r.parse::<i32>()?;
	}

	let mut answerp2 = 0;

	let do_dont_mul_re = regex::Regex::new(r"(do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\))")?;

	let mut mul_enabled = true;
	for (_, [m]) in do_dont_mul_re.captures_iter(&input0).map(|c| c.extract()) {
		match m {
			"do()" => mul_enabled = true,
			"don't()" => mul_enabled = false,
			mul => {
				if mul_enabled {
					let caps = mul_re.captures(mul).unwrap();
					let (_, [l, r]) = caps.extract();
					answerp2 += l.parse::<i32>()? * r.parse::<i32>()?;
				}
			}
		}
	}

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
