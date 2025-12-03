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
	let mut input0: Vec<Vec<u64>> = std::fs::read_to_string(filename)
		.context(format!("failed to read {filename} to string 2"))?
		.trim()
		.split('\n')
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
		.collect();

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	for line in input0.iter() {
		let mut biggest = 0;
		for i in 0..line.len() - 1 {
			let line = &line[i..];
			let l = 10 * line[0];
			for d in &line[1..] {
				let joltage = l + d;
				biggest = max(biggest, joltage);
			}
		}
		//println!("{line} = {biggest}");
		answerp1 += biggest;
	}

	const P2COUNT: usize = 12;

	for line in input0.iter() {
		let mut biggest = 0;
		let mut added = 0;
		let mut last = 0;
		let mut i = 0;
		while i < line.len() {
			let end = line.len() - (P2COUNT - added) + 1;
			let hay = &line[i..end];

			let mut pos = 0;
			let mut l = hay[0];
			for (i, d) in hay.iter().enumerate() {
				if *d > l {
					l = *d;
					pos = i;
				}
			}
			i += pos;
			biggest *= 10;
			biggest += l;
			added += 1;
			if added == 12 {
				break;
			}
			i += 1;
		}
		println!("{} = {biggest}", line.iter().map(|d| d.to_string()).join(""));
		answerp2 += biggest;
	}

	println!("\n\n====== {filename}:\n{answerp1}\n{answerp2}\n");

	Ok(())
}
