#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use core::cmp::Ordering;
use core::cmp::{max, min};
use core::str;
use std::{collections::HashMap, collections::HashSet, error::Error};

use anyhow::Context;
use itertools::{Itertools, rev, sorted};

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

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	for line in &grid {
		let line = unsafe { str::from_utf8_unchecked(line) };
		answerp1 += line.matches("XMAS").count();
		answerp1 += line.matches("SAMX").count();
	}

	let w = grid[0].len();
	let h = grid.len();

	// h-3 because we need space below for the verts
	for y in 0..h - 3 {
		for x in 0..w {
			let down = [grid[y + 0][x], grid[y + 1][x], grid[y + 2][x], grid[y + 3][x]];
			if down.eq(b"XMAS") || down.eq(b"SAMX") {
				// println!("down");
				answerp1 += 1;
			}
		}
	}
	for y in 3..h {
		for x in 3..w {
			let tl_to_br = [
				grid[y - 3][x - 3],
				grid[y - 2][x - 2],
				grid[y - 1][x - 1],
				grid[y - 0][x - 0],
			];
			if tl_to_br.eq(b"XMAS") || tl_to_br.eq(b"SAMX") {
				// println!("tl_to_br");
				answerp1 += 1;
			}
			let tr_to_bl = [
				grid[y - 3][x - 0],
				grid[y - 2][x - 1],
				grid[y - 1][x - 2],
				grid[y - 0][x - 3],
			];
			if tr_to_bl.eq(b"XMAS") || tr_to_bl.eq(b"SAMX") {
				// println!("tr_to_bl");
				answerp1 += 1;
			}
		}
	}

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
