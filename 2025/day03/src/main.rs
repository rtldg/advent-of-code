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

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	for (lineno, line) in input0.iter().enumerate() {
		let mut biggest = 0;
		for i in 0..line.len() - 1 {
			let line = &line[i..];
			let l = 10 * line.chars().next().unwrap().to_digit(10).unwrap();
			for c in (&line[1..]).chars() {
				let r = c.to_digit(10).unwrap();
				let joltage = l + r;
				biggest = max(biggest, joltage);
			}
		}
		println!("{line} = {biggest}");
		answerp1 += biggest;
	}

	println!("\n\n====== {filename}:\n{answerp1}\n{answerp2}\n");

	Ok(())
}
