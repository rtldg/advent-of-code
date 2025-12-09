#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

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
	let mut redtiles: Vec<[usize; 2]> = std::fs::read_to_string(filename)
		.context(format!("failed to read {filename} to string 2"))?
		.trim()
		.split('\n')
		.map(|line| line.split(',').map(|s| s.parse().unwrap()).collect_array().unwrap())
		.collect();

	let mut answerp1;
	let mut answerp2 = 0;

	let areas = redtiles
		.iter()
		.permutations(2)
		.par_bridge()
		.map(|v| {
			let (a, b) = if v[0][0] < v[1][0] {
				(v[0][0], v[1][0])
			} else {
				(v[1][0], v[1][0])
			};
			let (c, d) = if v[0][1] < v[1][1] {
				(v[0][1], v[1][1])
			} else {
				(v[1][1], v[1][1])
			};
			(b - a + 1) * (d - c + 1)
		})
		.collect_vec_list();
	let areas = areas.into_iter().flatten().collect_vec();

	answerp1 = *areas.iter().max().unwrap();

	//println!("{areas:?}");

	println!("\n\n====== {filename}:\n{answerp1}\n{answerp2}\n");

	Ok(())
}
