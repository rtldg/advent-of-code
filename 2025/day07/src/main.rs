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

fn start_splitter(rows: &mut [Vec<u8>], splitters: &mut HashSet<(usize, usize)>, starty: usize, startx: usize) {
	for y in starty..rows.len() {
		if rows[y][startx] == b'|' {
			return;
		}
		if rows[y][startx] == b'.' {
			rows[y][startx] = b'|';
		}
		if rows[y][startx] == b'^' {
			splitters.insert((y, startx));
			if rows[y].len() > 1 {
				start_splitter(rows, splitters, y, startx-1);
			}
			if startx < rows[y].len() {
				start_splitter(rows, splitters, y, startx+1);
			}
			break;
		}
	}
}

fn swag(filename: &str) -> anyhow::Result<()> {
	let mut input0 = std::fs::read_to_string(filename).context(format!("failed to read {filename} to string 1"))?;
	let mut input0: Vec<Vec<u8>> = std::fs::read_to_string(filename)
		.context(format!("failed to read {filename} to string 2"))?
		.trim()
		.split('\n')
		.map(|s| s.as_bytes().to_owned())
		.collect();

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	let S = input0[0].iter().position(|c| *c == b'S').unwrap();

	let mut splitters = HashSet::new();

	start_splitter(&mut input0, &mut splitters, 0, S);

	for line in input0 {
		println!("{}", str::from_utf8(&line).unwrap());
	}

	//println!("{splitters:?}");
	println!("{}", splitters.len());

	println!("\n\n====== {filename}:\n{answerp1}\n{answerp2}\n");

	Ok(())
}
