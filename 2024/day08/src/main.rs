#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use core::cmp::Ordering;
use core::cmp::{max, min};
use core::str;
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
		.lines()
		.map(|s| s.as_bytes().to_vec())
		.collect();

	let h = grid.len();
	let w = grid[0].len();

	let mut antinodes = vec![vec![b'.'; w]; h];

	let mut do_antinodes = |antinodes: &mut Vec<Vec<u8>>, anttype, starty, startx, p2| {
		for y in 0..h {
			for x in 0..w {
				if y == starty && x == startx {
					continue;
				}

				let v = grid[y][x];
				if v != anttype {
					continue;
				}

				let diffy = if starty > y {
					-((starty - y) as isize) as usize
				} else {
					y - starty
				};
				let diffx = if startx > x {
					-((startx - x) as isize) as usize
				} else {
					x - startx
				};

				let mut antiy = y + diffy;
				let mut antix = x + diffx;

				if p2 {
					antinodes[starty][startx] = anttype;
					antinodes[y][x] = anttype;
				}

				loop {
					if (0..h).contains(&antiy) && (0..w).contains(&antix) {
						// println!("plopping '{}' at y={} x={}", anttype as char, antiy, antix);
						antinodes[antiy][antix] = anttype;
						if p2 {
							antiy += diffy;
							antix += diffx;
						} else {
							break;
						}
					} else {
						break;
					}
				}
			}
		}
	};

	for y in 0..h {
		for x in 0..w {
			let antenna = grid[y][x];
			if antenna.is_ascii_alphanumeric() {
				do_antinodes(&mut antinodes, antenna, y, x, false);
			}
		}
	}

	let answerp1: usize = antinodes.iter().map(|r| r.iter().filter(|v| **v != b'.').count()).sum();

	for y in 0..h {
		for x in 0..w {
			let antenna = grid[y][x];
			if antenna.is_ascii_alphanumeric() {
				do_antinodes(&mut antinodes, antenna, y, x, true);
			}
		}
	}

	let answerp2: usize = antinodes.iter().map(|r| r.iter().filter(|v| **v != b'.').count()).sum();

	println!("");
	for row in &antinodes {
		println!("{}", str::from_utf8(row)?);
	}

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
