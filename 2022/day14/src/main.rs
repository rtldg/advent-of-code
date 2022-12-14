#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use core::cmp::{max, min};
use std::collections::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Slot {
    Air,
    Rock,
    Sand,
}
impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Slot::Air => '.',
                Slot::Rock => '#',
                Slot::Sand => 'o',
            }
        )
    }
}

fn hello(ispart2: bool) {
    // grid[x][y]
    let mut grid: Vec<Vec<Slot>> = vec![vec![Slot::Air; 1000]; 1000];

    let scans: Vec<Vec<Vec<usize>>> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(|x| {
            x.split("->")
                .map(|x| {
                    x.trim()
                        .split(',')
                        .map(|x| x.trim().parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut highesty = 0;

    for scan in scans {
        let (mut startx, mut starty) = (scan[0][0], scan[0][1]);
        for form in scan {
            let (endx, endy) = (form[0], form[1]);

            for x in min(startx, endx)..=max(startx, endx) {
                for y in min(starty, endy)..=max(starty, endy) {
                    grid[x][y] = Slot::Rock;
                }
            }

            highesty = max(highesty, max(starty, endy));
            (startx, starty) = (endx, endy);
        }
    }

    let floor = highesty + 2;

    if ispart2 {
        for x in 0..grid.len() {
            grid[x][floor] = Slot::Rock;
        }
    }

    let mut pieces_of_sand: usize = 0;
    'pour: loop {
        pieces_of_sand += 1;
        let (mut x, mut y) = (500, 0);

        'sand: loop {
            if !ispart2 && y > 666 {
                pieces_of_sand -= 1;
                break 'pour;
            }

            y += 1;

            if grid[x][y] == Slot::Air {
                continue 'sand;
            }

            if grid[x - 1][y] == Slot::Air {
                x -= 1;
                continue 'sand;
            } else if grid[x + 1][y] == Slot::Air {
                x += 1;
                continue 'sand;
            }

            // sand is trapped...
            grid[x][y - 1] = Slot::Sand;
            if x == 500 && y == 1 {
                break 'pour;
            }
            break 'sand;
        }
    }

    // for y in 0..=12 {
    // for x in 489..=511 {
    for y in 0..=floor {
        for x in 433..568 {
            // print!("{}", grid[x][y]);
        }
        // println!("");
    }

    println!("{}", pieces_of_sand);
}

fn main() {
    hello(false);
    hello(true);
}
