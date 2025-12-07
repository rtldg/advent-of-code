#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use core::cmp::Ordering;
use core::cmp::{max, min};
use std::time::Duration;
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
	swag2("test_input")?;
	swag2("input")?;
	Ok(())
}

fn start_splitter(rows: &mut [Vec<u8>], splitters: &mut HashSet<(usize, usize)>, starty: usize, startx: usize) {
	for y in starty..rows.len() {
		if rows[y][startx] == b'|' {
			return;
		} else if rows[y][startx] == b'.' {
			rows[y][startx] = b'|';
		} else if rows[y][startx] == b'^' {
			splitters.insert((y, startx));
			if rows[y].len() > 1 {
				start_splitter(rows, splitters, y, startx - 1);
			}
			if startx < rows[y].len() {
				start_splitter(rows, splitters, y, startx + 1);
			}
			return;
		}
	}
}

fn swag(filename: &str) -> anyhow::Result<()> {
	let mut input0: Vec<Vec<u8>> = std::fs::read_to_string(filename)
		.context(format!("failed to read {filename} to string 2"))?
		.trim()
		.split('\n')
		.map(|s| s.as_bytes().to_owned())
		.collect();

	let S = input0[0].iter().position(|c| *c == b'S').unwrap();

	let mut splitters = HashSet::new();

	start_splitter(&mut input0, &mut splitters, 0, S);

	for line in input0 {
		println!("{}", str::from_utf8(&line).unwrap());
	}

	let answer = splitters.len();

	println!("\n\n====== {filename}:\n{answer}\n");

	Ok(())
}

fn get_parents(rows: &[Vec<u8>], starty: usize, startx: usize) -> HashSet<(usize, usize)> {
	let mut set = HashSet::new();
	for y in (0..starty).rev() {
		if rows[y][startx] == b'^' {
			break;
		}
		if rows[y][startx - 1] == b'^' {
			set.insert((y, startx - 1));
		}
		if rows[y][startx + 1] == b'^' {
			set.insert((y, startx + 1));
		}
	}
	if starty == 14 && startx == 7 {
		println!("y{starty} x{startx}'s parents = {set:?}");
	}
	set
}

fn get_children(rows: &[Vec<u8>], starty: usize, startx: usize) -> Option<HashSet<(usize, usize)>> {
	let mut set = HashSet::new();
	let (mut l, mut r) = (false, false);
	for y in starty + 2..rows.len() {
		if !l {
			if rows[y][startx - 1] == b'^' {
				set.insert((y, startx - 1));
				l = true;
			}
		}
		if !r {
			if rows[y][startx + 1] == b'^' {
				set.insert((y, startx + 1));
				r = true;
			}
		}
		if l && r {
			break;
		}
	}
	if starty == 6 && startx == 7 {
		println!("y{starty} x{startx}'s children = {set:?}");
	}
	if set.is_empty() { None } else { Some(set) }
}

/*

Bottom splitters = 2

For each splitter in row:
	for each parent:
		parent.value += splitter.value

*/

fn swag2(filename: &str) -> anyhow::Result<()> {
	let mut input0: Vec<Vec<u8>> = std::fs::read_to_string(filename)
		.context(format!("failed to read {filename} to string 2"))?
		.trim()
		.split('\n')
		.map(|s| s.as_bytes().to_owned())
		.collect();

	let mut parents = HashMap::new();
	for y in (0..input0.len()).rev() {
		for x in 0..input0[y].len() {
			if input0[y][x] == b'^' {
				parents.insert((y, x), get_parents(&input0, y, x));
				//println!("y:{y} x:{x} = {parents:?}");
			}
		}
	}
	//std::thread::sleep(Duration::from_secs(10));

	let mut child_tracker = HashMap::new();
	for y in 0..input0.len() {
		for x in 0..input0[y].len() {
			if input0[y][x] == b'^' {
				if let Some(children) = get_children(&input0, y, x) {
					child_tracker.insert((y, x), children);
				}
			}
		}
	}

	let mut timelines = HashMap::new();
	for y in (4..input0.len()).step_by(2).rev() {
		for x in 0..input0[0].len() {
			if input0[y][x] == b'^' {
				//println!("y{y} x{x}");
				let my_value = *timelines.entry((y, x)).or_insert(2u64);
				for &(parenty, parentx) in parents.get(&(y, x)).unwrap() {
					//println!("giving {my_value} to y{parenty} x{parentx}");
					let children = child_tracker.get(&(parenty, parentx)).unwrap();
					let additional = if children.len() == 1 { 1 } else { 0 };
					*timelines.entry((parenty, parentx)).or_insert(0) += my_value + additional;

					/*
					for (y, line) in input0.iter().enumerate() {
						for (x, c) in line.iter().enumerate() {
							if *c == b'.' {
								print!("..");
							} else {
								if let Some(v) = timelines.get(&(y, x)) {
									print!("{v:02}")
								} else {
									print!("^^");
								}
							}
						}
						print!("\n");
					}
					//std::thread::sleep(Duration::from_secs(3));
					*/
				}
			}
		}
	}

	let Sx = input0[0].iter().position(|c| *c == b'S').unwrap();
	let answer = timelines.get(&(2, Sx)).unwrap();

	for (y, line) in input0.iter().enumerate() {
		for (x, c) in line.iter().enumerate() {
			if *c == b'.' {
				print!("..");
			} else {
				if let Some(v) = timelines.get(&(y, x)) {
					print!("{v:02}")
				} else {
					print!("^^");
				}
			}
		}
		print!("\n");
	}

	//println!("{parents:#?}");
	//println!("{:?}", parents.get(&(10, 5)));
	//println!("{:?}", child_tracker.get(&(6, 7)));

	println!("\n\n====== {filename}:\n{answer}\n");

	Ok(())
}

/*
fn iter_kids(child_tracker: &HashMap<(usize, usize), HashSet<(usize, usize)>>, startchild: (usize, usize)) -> usize {
	if let Some(children) = child_tracker.get(&startchild) {
		let mut c = if children.len() == 1 {
			1
		} else {
			2
		};
		for child in children {
			c += iter_kids(child_tracker, *child) - 1;
		}
		c
	} else {
		2
	}
}

fn swag2(filename: &str) -> anyhow::Result<()> {
	let mut input0: Vec<Vec<u8>> = std::fs::read_to_string(filename)
		.context(format!("failed to read {filename} to string 2"))?
		.trim()
		.split('\n')
		.map(|s| s.as_bytes().to_owned())
		.collect();

	let mut child_tracker = HashMap::new();

	for y in 0..input0.len() {
		for x in 0..input0[y].len() {
			if input0[y][x] == b'^' {
				if let Some(children) = get_children(&input0, y, x) {
					child_tracker.insert((y, x), children);
				}
			}
		}
	}

	for y in (0..input0.len()).rev() {
		for x in 0..input0[y].len() {
			if input0[y][x] == b'^' {
				let parents = get_parents(&input0, y, x);
				println!("y:{y} x:{x} = {parents:?}");
			}
		}
	}

	let Sx = input0[0].iter().position(|c| *c == b'S').unwrap();
	let first_splitter = (2, Sx);

	//child_tracker.get(&first_splitter).unwrap().iter().flat_map(f)

	//println!("{:?}", child_tracker.get(&first_splitter));

	//println!("{child_tracker:?}");

	let answer = iter_kids(&child_tracker, (2, Sx));

	println!("\n\n====== {filename}:\n{answer}\n");

	Ok(())
}
*/
