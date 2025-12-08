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

type Point = [f32; 3];

fn dist3d(a: &Point, b: &Point) -> f32 {
	f32::sqrt((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2))
}

fn swag(filename: &str) -> anyhow::Result<()> {
	let mut jboxes: Vec<Point> = std::fs::read_to_string(filename)
		.context(format!("failed to read {filename} to string 2"))?
		.trim()
		.split('\n')
		.map(|line| line.split(',').map(|s| s.parse().unwrap()).collect_array().unwrap())
		.collect();

	let mut answerp2 = 0;

	//let mut circuits: Vec<HashSet<Point>> = vec![];
	let mut circuits: Vec<Vec<Point>> = vec![];

	let dists = jboxes
		.iter()
		.combinations(2)
		.par_bridge()
		.map(|v| (v[0].clone(), v[1].clone(), dist3d(v[0], v[1])))
		.collect_vec_list();
	let mut dists = dists.into_iter().flatten().collect_vec();
	dists.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

	let pairs_to_take = if filename == "test_input" { 10 } else { 1000 };
	for &(a, b, dist) in dists.iter().take(pairs_to_take) {
		//println!("{a:?} <-> {b:?} = {dist}");
		let mut cir = vec![a, b];
		for v in circuits.extract_if(.., |v| v.contains(&a) || v.contains(&b)) {
			cir.extend(v);
		}
		cir.sort_by(|a, b| a.partial_cmp(&b).unwrap());
		cir.dedup();
		//println!("  cir.len() = {}", cir.len());
		circuits.push(cir);
	}

	let mut sizes: Vec<usize> = circuits.iter().map(|v| v.len()).collect();
	sizes.sort();
	let answerp1: usize = sizes.iter().rev().take(3).product();
	//println!("{circuits:#?}");
	println!("sizes = {sizes:?}");
	println!("unique = {}", circuits.iter().map(|v| v.len()).sum::<usize>());

	println!("\n\n====== {filename}:\n{answerp1}\n{answerp2}\n");

	Ok(())
}
