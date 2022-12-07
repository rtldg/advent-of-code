#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{collections::HashMap, path::PathBuf};

fn hello(ispart2: bool) {
    let mut input0 = std::fs::read_to_string("input").unwrap();
    let mut workingdir = std::path::PathBuf::new();
    let mut things = HashMap::new();

    for line in input0.split('\n') {
        let units: Vec<&str> = line.split(' ').collect();

        if units[0] == "$" {
            if units[1] == "cd" {
                match units[2] {
                    "/" => {
                        workingdir.clear();
                    },
                    "." => {
                        // nothing...
                    },
                    ".." => {
                        let _ = workingdir.pop();
                    },
                    _ => {
                        workingdir.push(units[2]);
                    },
                }
            }
        } else {
            if let Ok(filesize) = units[0].parse::<usize>() {
                workingdir.push(units[1]);
                let _ = things.insert(workingdir.to_str().unwrap().to_string(), filesize);
                workingdir.pop();
            } else {
                // "dir" ... nothing...
            }
        }
    }

    let mut dirsizes: HashMap<String, usize> = HashMap::new();

    for (key, val) in things.iter() {
        let mut x = PathBuf::from(key);
        let _ = x.pop();
        loop {
            let name = x.to_str().unwrap().to_string();
            *dirsizes.entry(name).or_default() += val;
            if !x.pop() { break; }
        }
    }

    println!("{:?}", dirsizes);

    let totalsz_p1: usize = dirsizes.values().filter(|x| **x <= 100000).sum();
    println!("{:?}", dirsizes.get(""));
    let p2_need = 30_000_000 - (70_000_000 - dirsizes.get("").unwrap());
    let totalsz_p2: usize = *dirsizes.values().filter(|x| **x >= p2_need).min().unwrap();

    println!("{}\n{}", totalsz_p1, totalsz_p2);
}

fn main() {
    hello(false);
    // hello(true);
}
