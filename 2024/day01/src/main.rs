#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use core::cmp::Ordering;
use core::cmp::{max, min};
use std::{collections::HashMap, collections::HashSet, error::Error};

use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	// let input_file = "test_input";
	let input_file = "input";

	let mut input0 = std::fs::read_to_string(input_file).context("failed to read input_file to string 1")?;
	let mut input0: Vec<String> = std::fs::read_to_string(input_file)
		.context("failed to read input_file to string 2")?
		.trim()
		.split('\n')
		.map(str::to_string)
		.collect();

	let mut answer = 0;

	let (mut leftlist, mut rightlist) = (vec![], vec![]);
	for (lineid, line) in input0.iter().enumerate() {
		let (l, r) = line.split_ascii_whitespace().collect_tuple().unwrap();
		leftlist.push(l.parse::<i32>()?);
		rightlist.push(r.parse::<i32>()?);
	}

	leftlist.sort();
	rightlist.sort();

	let distances: Vec<u32> = leftlist
		.iter()
		.zip(rightlist.iter())
		.map(|(l, r)| l.abs_diff(*r))
		.collect();

	let answer: u32 = distances.iter().sum();

	println!("\n\n{}\n\n", answer);

	Ok(())
}
