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
	// let input_file = "input";

	let mut input: Vec<i64> = std::fs::read_to_string(input_file)
		.context("failed to read input_file to string 2")?
		.trim()
		.split(' ')
		.map(|s| s.parse().unwrap())
		.collect();
	let mut inputp2 = input.clone();


	let mut answerp2 = 0;

	for blink in 0..25 {
		println!("blink = {blink} | input.len() = {}", input.len());
		// println!("{:?}", input[0]);
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

	let mut answerp1 = input.len();

	let mut counts = vec![];

	for stone in 0..=9999 {
		
	}

	let mut until_singles = vec![];
	for stone in &inputp2 {
		let mut stones = vec![*stone];
		for blink in 0..75 {
			println!("{blink}");
			let mut i = 0;
			while i < stones.len() {
				let stone = stones[i];
				if stone == 0 {
					stones[i] = 1;
				} else {
					let digits = stone.to_string();
					if digits.len() & 1 == 0 {
						let (l, r) = digits.split_at(digits.len() / 2);
						stones[i] = l.parse()?;
						stones.insert(i+1, r.parse()?);
						i += 1;
					} else {
						stones[i] *= 2024;
					}
				}
				if stones.iter().find(|v| **v > 9).is_none() {
					until_singles.push(blink);
					println!("found single for {stone} at {blink}");
					break;
				}
				i += 1;
			}
		}
	}

	println!("{:?}", until_singles);

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
