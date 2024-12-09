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
	// let input_file = "input";

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	let mut grid: String = std::fs::read_to_string(input_file)
		.context("failed to read input_file to string 2")?
		.trim();

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
