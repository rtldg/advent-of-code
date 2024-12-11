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

	let mut input: Vec<i64> = std::fs::read_to_string(input_file)
		.context("failed to read input_file to string 2")?
		.trim()
		.split(' ')
		.map(|s| s.parse().unwrap())
		.collect();

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	for blink in 0..25 {
		let mut i = 0;
		while i < input.len() {
			let stone = input[i];
			if stone == 0 {
				input[i] = 1;
			} else {
				let digits = stone.to_string();
				if digits.len() & 1 == 0 {
					let (l, r) = digits.split_at(digits.len() / 2);
					input[i] = l.parse()?;
					input.insert(i+1, r.parse()?);
					i += 1;
				} else {
					input[i] *= 2024;
				}
			}
			i += 1;
		}
	}

	answerp1 = input.len();

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
