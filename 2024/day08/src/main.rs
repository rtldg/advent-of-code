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

	let mut answerp2 = 0;

	let mut grid: Vec<Vec<u8>> = std::fs::read_to_string(input_file)
		.context("failed to read input_file to string 2")?
		.trim()
		.lines()
		.map(|s| s.as_bytes().to_vec())
		.collect();

	let h = grid.len();
	let w = grid[0].len();

	let mut antinodes = vec![vec![b'.'; w]; h];

	let mut real_antinodes = HashMap::<(usize, usize, u8), bool>::new();

	let mut do_antinodes = |anttype, starty, startx| {
		for y in 0..h {
			for x in 0..w {
				if y == starty && x == startx {
					continue;
				}

				let v = grid[y][x];
				if v != anttype {
					continue;
				}

				let antiy = if starty > y {
					y - (starty - y)
				} else {
					y + (y - starty)
				};
				let antix = if startx > x {
					x - (startx - x)
				} else {
					x + (x - startx)
				};

				if (0..h).contains(&antiy) && (0..w).contains(&antix) {
					println!("plopping '{}' at y={} x={}", anttype as char, antiy, antix);
					antinodes[antiy][antix] = anttype;
					let _ = real_antinodes.insert((antiy, antix, anttype), true);
				}
			}
		}
	};

	for y in 0..h {
		for x in 0..w {
			let antenna = grid[y][x];
			if antenna.is_ascii_alphanumeric() {
				do_antinodes(antenna, y, x);
			}
		}
	}

	let answerp1: usize = antinodes.iter().map(|r| r.iter().filter(|v| **v != b'.').count()).sum();

	println!("");
	for y in 0..h {
		for x in 0..w {
			print!("{}", antinodes[y][x] as char);
		}
		println!("");
	}

	// let answerp1 = real_antinodes.len();

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
