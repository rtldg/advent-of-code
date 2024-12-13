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
	// regular input is NOT 1460306

	let mut grid: Vec<Vec<u8>> = std::fs::read_to_string(input_file)
		.context("failed to read input_file to string 2")?
		.trim()
		.split('\n')
		.map(|s| s.as_bytes().to_vec())
		// .map(str::to_string)
		.collect();

	grid.insert(0, vec![0; grid[0].len()]);
	grid.push(vec![0; grid[0].len()]);

	for r in grid.iter_mut() {
		r.insert(0, 0);
		r.push(0);
	}

	let h = grid.len();
	let w = grid[0].len();

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	let mut plants = HashMap::<char, Vec<(usize, usize)>>::new();

	let mut visited = vec![vec![false; w]; h];
	let mut fences = vec![vec![false; w * 2]; h * 2];

	fn traverse(
		grid: &Vec<Vec<u8>>,
		visited: &mut Vec<Vec<bool>>,
		fences: &mut Vec<Vec<bool>>,
		target_plant: u8,
		y: usize,
		x: usize,
	) -> usize {
		if visited[y][x] {
			return 0;
		}
		visited[y][x] = true;

		let mut area = 1;

		// up
		if grid[y - 1][x] != target_plant {
			fences[y * 2 + 0][x * 2 + 0] = true;
			fences[y * 2 + 0][x * 2 + 2] = true;
		} else {
			area += traverse(grid, visited, fences, target_plant, y - 1, x);
		}
		// down
		if grid[y + 1][x] != target_plant {
			fences[y * 2 + 2][x * 2 + 0] = true;
			fences[y * 2 + 2][x * 2 + 2] = true;
		} else {
			area += traverse(grid, visited, fences, target_plant, y + 1, x);
		}
		// left
		if grid[y][x - 1] != target_plant {
			fences[y * 2 + 0][x * 2 + 0] = true;
			fences[y * 2 + 2][x * 2 + 0] = true;
		} else {
			area += traverse(grid, visited, fences, target_plant, y, x - 1);
		}
		// right
		if grid[y][x + 1] != target_plant {
			fences[y * 2 + 0][x * 2 + 2] = true;
			fences[y * 2 + 2][x * 2 + 2] = true;
		} else {
			area += traverse(grid, visited, fences, target_plant, y, x + 1);
		}

		area
	}

	for y in 1..h - 1 {
		for x in 1..w - 1 {
			if visited[y][x] {
				continue;
			}

			let target_plant = grid[y][x];

			let area = traverse(&grid, &mut visited, &mut fences, grid[y][x], y, x);

			/*
			for row in 0..fences.len() {
				for column in 0..fences[0].len() {
					if fences[row][column] {
						print!("+");
					} else {
						print!(".");
					}
				}
				print!("\n");
			}
			print!("\n");
			*/

			let fence_count = fences.iter().map(|r| r.iter().filter(|c| **c).count()).sum();
			plants
				.entry(target_plant as char)
				.or_default()
				.push((area, fence_count));

			for r in fences.iter_mut() {
				r.fill(false);
			}
		}
	}

	println!("{:#?}", plants);

	let answerp1: usize = plants
		.iter()
		.map(|(plant, v)| v.iter().map(|(a, f)| *a * f).sum::<usize>())
		.sum();

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
