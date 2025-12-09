#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use core::cmp::Ordering;
use core::cmp::{max, min};
use std::{collections::HashMap, collections::HashSet, error::Error};

use anyhow::Context;
use fixedbitset::FixedBitSet;
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
	redtiles.push(redtiles[0]); // close loop

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

	/* Pseudo-code:

	areas.sort_by(|a,b | b.2.cmp(a.2));
	for (a, b, area) in areas.iter() {
		let mut blocks = vec![(a, b)];

		while let Some(block) = blocks.pop() {
			let mut covered = false;
			for thing in redtiles.perms(2) {
				if overlap {
					covered = true;
					for piece in non_overlapped {
						blocks.push(piece)
					}
					break;
				}
			}
			if covered && blocks.is_empty() {
				answerp2 = max(answerp2, area_of_a_and_b);
			}
		}
	}
	*/

	/*
	let square_width = 1 + if filename == "test_input" { 12 } else { 100_000 };

	let mut rows = vec![];
	for i in 0..square_width {
		rows.push(FixedBitSet::with_capacity(square_width));
	}

	for win in redtiles.windows(2) {
		let (a, b) = if win[0][0] < win[1][0] {
			(win[0][0], win[1][0])
		} else {
			(win[1][0], win[0][0])
		};
		let (c, d) = if win[0][1] < win[1][1] {
			(win[0][1], win[1][1])
		} else {
			(win[1][1], win[0][1])
		};
		if win[0][0] == win[1][0] {
			// same x
			for y in c..d + 1 {
				rows[y].set(a, true);
			}
		} else {
			// same y
			for x in a..b + 1 {
				rows[c].set(x, true);
			}
		}
	}

	// fill the bits next :sob:

	let guessed_center = if filename == "test_input" {
		(4, 4)
	} else {
		(40_000, 40_000)
	};

	// (y, x) // down, right, up, left....
	let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

	let mut queue = vec![guessed_center];

	while !queue.is_empty() {
		let curpos = queue.pop().unwrap();
		for (dir, &(diry, dirx)) in dirs.iter().enumerate() {
			let pos = ((curpos.0 as isize + diry) as usize, (curpos.1 as isize + dirx) as usize);
			if rows[pos.0][pos.1] == false {
				rows[pos.0].set(pos.1, true);
				queue.push(pos);
			}
		}
	}

	if filename == "test_input" {
		for row in rows {
			for x in 0..row.len() {
				if row[x] {
					print!("X");
				} else {
					print!(".");
				}
			}
			print!("\n");
		}
	}
	*/

	//println!("{areas:?}");
	println!("{}", areas.len());

	println!("\n\n====== {filename}:\n{answerp1}\n{answerp2}\n");

	Ok(())
}
