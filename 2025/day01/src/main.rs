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

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	let mut dial = 50;

	for (lineno, line) in input0.iter().enumerate() {
		let rotations = i32::from_str_radix(&line[1..], 10)?;

		if line.starts_with('L') {
			dial = (dial - rotations) % 100;
		} else {
			dial = (dial + rotations) % 100;
		}

		if dial == 0 {
			answerp1 += 1;
		}
	}

	println!("\n\n====== {filename}:\n{answerp1}\n\n{answerp2}\n\n");

	Ok(())
}
