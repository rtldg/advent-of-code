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
use regex::Regex;

fn main() -> anyhow::Result<()> {
	let input_file = "test_input";
	let input_file = "input";

	let mut input: String = std::fs::read_to_string(input_file).context("failed to read input_file to string 2")?;

	let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)")?;

	let mut answerp1 = 0;
	for caps in re.captures_iter(&input) {
		let (_, [ax, ay, bx, by, px, py]) = caps.extract();
		let (ax, ay, bx, by, px, py): (usize, usize, usize, usize, usize, usize) = (
			ax.parse()?,
			ay.parse()?,
			bx.parse()?,
			by.parse()?,
			px.parse()?,
			py.parse()?,
		);
		let mut lowest = core::usize::MAX;
		for a in 0..=100 {
			for b in 0..=100 {
				if a * ax + b * bx == px && a * ay + b * by == py {
					lowest = lowest.min(a * 3 + b);
				}
			}
		}
		if lowest != core::usize::MAX {
			answerp1 += lowest;
		}
	}

	// part 2 moment:
	// >Unfortunately, it will take many more than 100 presses to do so.
	// it's over for bruteforcers...

	let mut answerp2 = 0;
	for caps in re.captures_iter(&input) {
		let (_, [ax, ay, bx, by, px, py]) = caps.extract();
		let (ax, ay, bx, by, px, py): (usize, usize, usize, usize, usize, usize) = (
			ax.parse()?,
			ay.parse()?,
			bx.parse()?,
			by.parse()?,
			px.parse()?,
			py.parse()?,
		);
		let px = px + 10000000000000;
		let py = py + 10000000000000;
		let mut lowest = core::usize::MAX;
		for a in 0..=100 {
			for b in 0..=100 {
				if a == 0 && b == 0 {
					continue;
				}
				let xdist = a * ax + b * bx;
				let ydist = a * ay + b * by;
				// println!("{px} / {xdist}, {py} / {ydist}");
				let divisor = max(px / xdist, py / ydist);
				lowest = lowest.min(a * divisor * 3 + b * divisor);
			}
		}
		if lowest != core::usize::MAX {
			answerp2 += lowest;
		}
	}

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
