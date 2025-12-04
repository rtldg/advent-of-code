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
	let mut board: Vec<Vec<u8>> = std::fs::read_to_string(filename)
		.context(format!("failed to read {filename} to string 2"))?
		.trim()
		.split('\n')
		.map(|line| line.as_bytes().to_owned())
		.collect();

	let mut battleships = vec![vec![false; board[0].len()]; board.len()];
	//let mut battleships = board.clone();

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	let adjacent_coords = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

	for y in 0..board.len() {
		for x in 0..board[0].len() {
			if board[y][x] != b'@' {
				continue;
			}

			let mut adjacent = 0;
			for (yoff, xoff) in adjacent_coords {
				if let Some(row) = board.get((y as isize + yoff) as usize)
					&& let Some(point) = row.get((x as isize + xoff) as usize)
				{
					if *point == b'@' {
						adjacent += 1;
					}
				}
			}
			if adjacent < 4 {
				//println!("y:{y} x:{x}");
				//battleships[y][x] = true;
				answerp1 += 1;
			}
		}
	}

	loop {
		let mut deforested = false;

		for y in 0..board.len() {
			for x in 0..board[0].len() {
				if board[y][x] != b'@' {
					continue;
				}

				let mut adjacent = 0;
				for (yoff, xoff) in adjacent_coords {
					if let Some(row) = board.get((y as isize + yoff) as usize)
						&& let Some(point) = row.get((x as isize + xoff) as usize)
					{
						if *point == b'@' {
							adjacent += 1;
						}
					}
				}
				if adjacent < 4 {
					board[y][x] = b'.';
					//println!("y:{y} x:{x}");
					//battleships[y][x] = true;
					answerp2 += 1;
					deforested = true;
				}
			}
		}

		if !deforested {
			break;
		}
	}

	println!("\n\n====== {filename}:\n{answerp1}\n{answerp2}\n");

	Ok(())
}
