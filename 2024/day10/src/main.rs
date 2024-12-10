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

	let mut grid: Vec<Vec<u8>> = std::fs::read_to_string(input_file)
		.context("failed to read input_file to string 2")?
		.trim()
		.split('\n')
		.map(|s| s.as_bytes().to_vec())
		// .map(str::to_string)
		.collect();

	let h = grid.len();
	let w = grid[0].len();

	let mut scores = HashMap::new();
	let mut scores2 = HashMap::new();

	fn traverse(
		grid: &Vec<Vec<u8>>,
		scores: &mut HashMap<((usize, usize), (usize, usize)), bool>,
		scores2: &mut HashMap<(usize, usize), usize>,
		trailhead: (usize, usize),
		y: usize,
		x: usize,
	) {
		let directions = [[-1isize as usize, 0], [0, 1], [1, 0], [0, -1isize as usize]];
		let current_height = grid[y][x];
		for dir in &directions {
			let nexty = y + dir[0];
			let nextx = x + dir[1];
			if (0..grid.len()).contains(&nexty) && (0..grid[0].len()).contains(&nextx) {
				let next = grid[nexty][nextx];
				if next.is_ascii_alphanumeric() && current_height + 1 == next {
					if next == b'9' {
						*scores.entry((trailhead, (nexty, nextx))).or_default() = true;
						*scores2.entry(trailhead).or_default() += 1;
						// println!("y={nexty} x={nextx}  - count={}", scores.values().count());
					} else {
						traverse(&grid, scores, scores2, trailhead, nexty, nextx);
					}
				}
			}
		}
	}

	for y in 0..h {
		for x in 0..w {
			if grid[y][x] == b'0' {
				traverse(&grid, &mut scores, &mut scores2, (y, x), y, x);
			}
		}
	}

	let answerp1: usize = scores.values().count();
	let answerp2: usize = scores2.values().sum();

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
