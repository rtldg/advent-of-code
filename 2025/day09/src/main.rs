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

fn get_area(a: &[usize; 2], b: &[usize; 2]) -> usize {
	let (minx, maxx) = if a[0] < b[0] { (a[0], b[0]) } else { (b[0], a[0]) };
	let (miny, maxy) = if a[1] < b[1] { (a[1], b[1]) } else { (b[1], a[1]) };
	(maxx - minx + 1) * (maxy - miny + 1)
}

fn boxes_intersect(a: &[usize; 2], b: &[usize; 2]) -> bool {
	let (minx, maxx) = if a[0] < b[0] { (a[0], b[0]) } else { (b[0], a[0]) };
	let (miny, maxy) = if a[1] < b[1] { (a[1], b[1]) } else { (b[1], a[1]) };
	maxx >= minx && maxy >= miny &&
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
	let mut answerp2;

	let areas = redtiles
		.iter()
		.permutations(2)
		.par_bridge()
		.map(|v| (*v[0], *v[1], get_area(v[0], v[1])))
		.collect_vec_list();
	let mut areas = areas.into_iter().flatten().collect_vec();

	answerp1 = areas.iter().map(|a| a.2).max().unwrap();

	// reverse sort
	areas.sort_by(|a, b| {
		b.2.cmp(&a.2)
	});

	let p2areas = areas.par_iter().map(|&(a, b, area)| {
		let mut blocks = vec![(a, b)];

		while let Some(block) = blocks.pop() {
			let mut covered = false;
			for tiles in redtiles.iter().permutations(2) {
				let newblock = (*tiles[0], *tiles[1]);
				if boxes_intersect(&a, &b)
			}
			if covered && blocks.is_empty() {
				return get_area(&a, &b);
			}
		}
		0
	}).collect_vec_list();
	let p2areas = p2areas.into_iter().flatten().collect_vec();

	answerp2 = *p2areas.iter().max().unwrap();

	/*
	- Fill array of lines
	- Merge lines
	- Check if box borders are on lines -- NOPE, won't work.  Might actually have to flood fill :pensive:
	*/

	/*
	- Fill array of lines
	- Merge lines


	- Calculate boxes based on 3 points.
	  But how do we detect if it's inline?
	*/

	/*
	For each box:
	- check if lines span
	*/

	/* Pseudo-code:

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
