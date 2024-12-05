#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use core::cmp::Ordering;
use core::cmp::{max, min};
use std::{collections::HashMap, collections::HashSet, error::Error};

use anyhow::Context;
use itertools::{Itertools, rev, sorted};

fn main() -> anyhow::Result<()> {
	let input_file = "test_input";
	let input_file = "input";

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	let input0 = std::fs::read_to_string(input_file).context("failed to read input_file to string 1")?;

	let (page_ordering_rules, page_update_set) = input0.trim().split("\n\n").collect_tuple().unwrap();

	let mut rules: Vec<(i32, i32)> = vec![];
	// let mut rule_map = HashMap::<i32, Vec<i32>>::new();
	for line in page_ordering_rules.split('\n') {
		let (l, r) = line.split('|').map(|s| s.parse().unwrap()).collect_tuple().unwrap();
		// rule_map.entry(l).or_default().push(r);
		rules.push((l, r));
	}

	let mut page_update_set: Vec<Vec<i32>> = page_update_set
		.trim()
		.split('\n')
		.map(|line| line.split(',').map(|s| s.parse::<i32>().unwrap()).collect())
		.collect();

	let mut bad_updates = vec![];
	'next_update: for update in page_update_set.iter() {
		for i in 0..update.len() {
			let page = update[i];
			for (l, r) in &rules {
				if page == *l {
					if let Some(pos) = (&update[..i]).iter().find_position(|v| **v == *r) {
						// not good
						bad_updates.push(update.clone());
						continue 'next_update;
					}
				}
			}
		}
		println!("good with {update:?}");
		answerp1 += update[update.len() / 2];
	}

	println!("");

	for update in bad_updates.iter_mut() {
		let mut uidx = 0;
		'restart_page: while uidx < update.len() {
			let page = update[uidx];
			for (l, r) in &rules {
				if page == *l {
					if let Some((pos, _)) = (&update[..uidx]).iter().find_position(|v| **v == *r) {
						println!("attempting to swap with ({l}|{r}) {update:?}?");
						update.swap(uidx, pos);
						uidx = 0;
						continue 'restart_page;
					}
				}
			}
			uidx += 1;
		}
		println!("bad with {update:?}");
		answerp2 += update[update.len() / 2];
	}

	println!("\n\n{}\n\n{}\n\n", answerp1, answerp2);

	Ok(())
}
