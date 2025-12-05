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
	let (fresh, available) = input0.split("\n\n").collect_tuple().unwrap();
	let fresh: Vec<_> = fresh
		.trim()
		.split('\n')
		.map(|line| {
			let (l, r) = line
				.split('-')
				.map(|s| s.parse::<u64>().unwrap())
				.collect_tuple()
				.unwrap();
			l..=r
		})
		.collect();
	let available: Vec<u64> = available.trim().split('\n').map(|l| l.parse().unwrap()).collect();

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	for a in &available {
		for range in &fresh {
			if range.contains(a) {
				//println!("{range:?} ({a}");
				answerp1 += 1;
				break;
			}
		}
	}

	println!("\n\n====== {filename}:\n{answerp1}\n{answerp2}\n");

	Ok(())
}
