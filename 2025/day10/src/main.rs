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
	//swag("input")?;
	Ok(())
}

#[derive(Debug)]
struct Machine {
	lights: u32,
	light_goal: u32,
	buttons: Vec<u32>,
	joltage_requirements: Vec<u32>,
}

fn swag(filename: &str) -> anyhow::Result<()> {
	let mut input0 = std::fs::read_to_string(filename).context(format!("failed to read {filename} to string 1"))?;
	let mut input0: Vec<String> = std::fs::read_to_string(filename)
		.context(format!("failed to read {filename} to string 2"))?
		.trim()
		.split('\n')
		.map(str::to_string)
		.collect();

	let mut answerp1 = 0;
	let mut answerp2 = 0;

	let mut machines = vec![];

	for (lineno, line) in input0.iter().enumerate() {
		let splits = line.split_ascii_whitespace().collect_vec();
		//let joltagereq = splits[splits.len()-1];

		let light_goal_s = &splits[0][1..splits[0].len()-1];
		let mut light_goal_u32 = 0;
		let lights = light_goal_s.len() as u32;
		for (i, c) in light_goal_s.chars().enumerate() {
			if c == '#' {
				light_goal_u32 |= 1 << i;
			}
		}

		let mut buttons = vec![];
		for button in &splits[1..splits.len() - 1] {
			let s = &button[1..button.len()-1];
			for button in s.split(',') {
				buttons.push(1 << button.parse::<u32>().unwrap());
			}
		}
		let buttons = buttons.iter().cycle().cloned().take(buttons.len() * 5).collect();

		machines.push(Machine {
			lights: lights,
			light_goal: light_goal_u32,
			buttons: buttons,
			joltage_requirements: vec![],
		});
	}

	println!("{machines:?}");

	for machine in machines {
		let x = machine.buttons.iter().permutations(5).par_bridge().map(|perm| {
			let mut state = 0;
			for (i, b) in perm.iter().enumerate() {
				state ^= *b;
				if state == machine.light_goal {
					return i + 1;
				}
			}
			1000
		}).collect_vec_list();
		let x = x.iter().flatten().cloned().collect_vec();
		println!("{}", x.iter().min().unwrap());
	}

	println!("\n\n====== {filename}:\n{answerp1}\n{answerp2}\n");

	Ok(())
}
