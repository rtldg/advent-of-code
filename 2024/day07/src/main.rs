#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use core::cmp::Ordering;
use core::cmp::{max, min};
use std::sync::atomic::AtomicU64;
use std::{collections::HashMap, collections::HashSet, error::Error};

use anyhow::Context;
use itertools::{Itertools, rev, sorted};
use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
	let input_file = "test_input";
	let input_file = "input";

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	let mut lines: Vec<String> = std::fs::read_to_string(input_file)
		.context("failed to read input_file to string 2")?
		.trim()
		.lines()
		.map(str::to_string)
		.collect();

	for line in &lines {
		let (target, args) = line.split(':').collect_tuple().unwrap();
		let target: i64 = target.parse()?;
		let args: Vec<i64> = args
			.trim()
			.split_ascii_whitespace()
			.map(|s| s.parse().unwrap())
			.collect();
		println!("{target}: {args:?}");
		let last_answerp1 = answerp1;
		for perm in itertools::repeat_n(0..=1, args.len() - 1).multi_cartesian_product() {
			let mut result = args[0];
			for i in 1..args.len() {
				if perm[i - 1] == 0 {
					result += args[i];
				} else {
					result *= args[i];
				}
			}
			if result == target {
				println!("  found");
				answerp1 += result;
				break;
			}
		}
		if answerp1 == last_answerp1 {
			for perm in itertools::repeat_n(0..=2, args.len() - 1).multi_cartesian_product() {
				let mut result = args[0];
				for i in 1..args.len() {
					match perm[i - 1] {
						0 => result += args[i],
						1 => result *= args[i],
						2 => result = (result.to_string() + &args[i].to_string()).parse()?,
						_ => (),
					};
				}
				if result == target {
					println!("  found");
					answerp2 += result;
					break;
				}
			}
		}
	}

	answerp2 += answerp1;

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
