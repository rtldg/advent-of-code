#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use core::cmp::Ordering;
use core::cmp::{max, min};
use std::{collections::HashMap, collections::HashSet, error::Error};

use anyhow::Context;
use itertools::Itertools;
use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
	unsafe {
		std::env::set_var("RUST_BACKTRACE", "full");
	}
	swag("test_input")?;
	swag("input")?;
	Ok(())
}

fn swag(filename: &str) -> anyhow::Result<()> {
	let mut input0 = std::fs::read_to_string(filename).context(format!("failed to read {filename} to string 1"))?;
	let mut input0: Vec<String> = std::fs::read_to_string(filename)
		.context(format!("failed to read {filename} to string 2"))?
		.trim()
		.split('\n')
		.map(str::to_string)
		.collect();

	let mut answerp1 = 0u64;
	let mut answerp2 = 0u64;

	let mut numbers = vec![];
	let mut operators = vec![];
	let mut ncol = input0[0].trim().split_ascii_whitespace().count();
	numbers.resize(ncol, vec![]);

	for (lineno, line) in input0.iter().enumerate() {
		let line = line.trim();
		let mut contents = line.split_ascii_whitespace();
		if line.as_bytes()[0].is_ascii_digit() {
			for (i, v) in contents.enumerate() {
				numbers[i].push(v.parse::<u64>().unwrap());
			}
		} else {
			operators = contents.map(|s| s.as_bytes()[0]).collect();
		}
	}

	for (i, op) in operators.iter().enumerate() {
		if *op == b'+' {
			answerp1 += numbers[i].iter().sum::<u64>();
		} else if *op == b'*' {
			answerp1 += numbers[i].iter().product::<u64>();
		}
	}

	//dbg!(numbers);
	//dbg!(operators);

	println!("\n\n====== {filename}:\n{answerp1}\n{answerp2}\n");

	Ok(())
}
