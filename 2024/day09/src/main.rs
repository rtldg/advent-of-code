#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use core::cmp::Ordering;
use core::cmp::{max, min};
use core::str;
use std::sync::atomic::AtomicU64;
use std::u16;
use std::{collections::HashMap, collections::HashSet, error::Error};

use anyhow::Context;
use itertools::{Itertools, rev, sorted};
use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
	let input_file = "test_input";
	let input_file = "input";

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	let mut diskmap = std::fs::read_to_string(input_file).context("failed to read input_file to string 2")?;

	let mut disk: Vec<u16> = vec![];

	for (charidx, c) in diskmap.chars().enumerate() {
		let count = c.to_digit(10).unwrap();
		if (charidx & 1) == 0 {
			// number of blocks in file
			let fileid = charidx / 2;
			for i in 0..count {
				disk.push(fileid as u16);
			}
		} else {
			// number of empty blocks
			for i in 0..count {
				// 0xFFFF / u16::MAX is used because the input is 20k~ characters. Divide by 2 to get 10k which doesn't fit in a u8 but it does fit in a u16...
				// We need a flag and we won't legitimately hit 0xFFFF.
				disk.push(u16::MAX)
			}
		}
	}

	// println!("{:?}", disk);

	for i in 0..disk.len() {
		if disk[i] == u16::MAX {
			let non_empty = disk.iter().rposition(|v| *v != u16::MAX).unwrap();

			if non_empty == i || non_empty < i {
				break;
			}

			disk.swap(i, non_empty);
			// println!("{:?} {i} {non_empty}", disk);
		}
	}

	// println!("{:?}", disk);

	for (i, v) in disk.iter().enumerate() {
		if *v == u16::MAX {
			break;
		}

		answerp1 += i * (*v as usize);
	}

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
