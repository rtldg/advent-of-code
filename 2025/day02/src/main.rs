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

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	for pid_range in input0.trim().split(',') {
		let (l, r) = pid_range.split('-').collect_tuple().unwrap();
		let l: u64 = l.parse().unwrap();
		let r: u64 = r.parse().unwrap();

		for i in max(10, l)..=r {
			let x = i.to_string();
			if x.len() % 2 != 0 {
				continue;
			}
			let (a, b) = x.split_at(x.len()/2);
			if a == b {
				answerp1 += i;
			}
		}

		//println!("{}", r-l);
	}

	println!("\n\n====== {filename}:\n{answerp1}\n{answerp2}\n");

	Ok(())
}
