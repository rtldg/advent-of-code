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

	let mut answerp1 = 0;
	let mut answerp2 = AtomicU64::new(0);

	let mut grid: Vec<Vec<u8>> = std::fs::read_to_string(input_file)
		.context("failed to read input_file to string 2")?
		.trim()
		.split('\n')
		.map(|s| s.as_bytes().to_vec())
		// .map(str::to_string)
		.collect();

	let w = grid[0].len();
	let h = grid.len();

	// let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];
	let mut visited = HashMap::<(usize, usize), bool>::new();

	let (mut guard_init_x, mut guard_init_y) = (0, 0);

	for (y, row) in grid.iter_mut().enumerate() {
		if let Some(x) = row.iter().position(|c| *c == b'^') {
			guard_init_x = x;
			guard_init_y = y;
			row[x] = b'.';
			// visited[y][x] = true;
			break;
		}
	}

	// up, right, down, left
	// [y,x]
	let dir_translation = [[-1isize as usize, 0], [0, 1], [1, 0], [0, -1isize as usize]];

	let mut dir = 0;
	let (mut guard_x, mut guard_y) = (guard_init_x, guard_init_y);

	loop {
		// visited[guard_y][guard_x] = true;
		let _ = visited.insert((guard_y, guard_x), true);

		let next_y = dir_translation[dir][0] + guard_y;
		let next_x = dir_translation[dir][1] + guard_x;

		if !(0..w).contains(&next_x) || !(0..h).contains(&next_y) {
			break;
		}

		if grid[next_y][next_x] == b'#' {
			dir = (dir + 1) % 4;
		} else {
			guard_y = next_y;
			guard_x = next_x;
		}
	}

	answerp1 = visited.len();

	// for obs_y in 0..h {
	(0..h).into_par_iter().for_each(|obs_y| {
		for obs_x in 0..w {
			if guard_init_x == obs_x && guard_init_y == obs_y {
				continue;
			}

			let mut dir = 0;
			let (mut guard_x, mut guard_y) = (guard_init_x, guard_init_y);
			let mut visited_with_dir = HashMap::<(usize, usize, usize), bool>::new();

			loop {
				if let Some(_) = visited_with_dir.insert((guard_y, guard_x, dir), true) {
					// we looped!
					// println!("placed with y={obs_y} x={obs_x} dir={dir}");
					answerp2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
					break;
				}

				let next_y = dir_translation[dir][0] + guard_y;
				let next_x = dir_translation[dir][1] + guard_x;

				if !(0..w).contains(&next_x) || !(0..h).contains(&next_y) {
					break;
				}

				if grid[next_y][next_x] == b'#' || (next_y == obs_y && next_x == obs_x) {
					dir = (dir + 1) % 4;
				} else {
					guard_y = next_y;
					guard_x = next_x;
				}
			}
		}
	});

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2.load(std::sync::atomic::Ordering::SeqCst));

	Ok(())
}
