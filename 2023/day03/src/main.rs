#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{collections::HashSet, error::Error};

fn is_symbol(c: char) -> bool {
    match c {
        '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '\\' | '/' | '+' | '-' | '=' => true,
        _ => false,
    }
}

fn adjacent_symbols(map: &Vec<Vec<u8>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut symbols = vec![];
    for thing in [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ] {
        let (a, b) = (
            (y as isize + thing.1) as usize,
            (x as isize + thing.0) as usize,
        );
        if is_symbol(map[a][b] as char) {
            symbols.push((a, b));
        }
    }
    symbols
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    let input_file = "input";
    let input0 = std::fs::read_to_string(input_file)?;

    let mut input0: Vec<Vec<u8>> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .map(|s| s.as_bytes().into())
        .collect();

    // simplify bounds checking...
    let (width, height) = (input0[0].len(), input0.len());
    input0.insert(0, vec![0; width]);
    input0.push(vec![0; width]);
    let height = height + 2;
    for y in 0..height {
        input0[y].insert(0, 0);
        input0[y].push(0);
    }
    let width = width + 2;

    let mut symbol_thing = vec![vec![vec![]; width]; height];

    let mut part_number_sum = 0;

    for y in 0..height {
        let mut numbuf = String::new();
        let mut adjacent = vec![];

        for x in 0..width {
            let c = input0[y][x] as char;
            if c.is_ascii_digit() {
                if adjacent.len() == 0 {
                    adjacent.append(&mut adjacent_symbols(&input0, x, y))
                }
                numbuf.push(c);
            } else {
                if adjacent.len() > 0 {
                    let part_number = numbuf.parse::<usize>().unwrap();
                    let unique = adjacent
                        .clone()
                        .into_iter()
                        .collect::<HashSet<(usize, usize)>>();
                    for u in unique {
                        // if input0[u.0][u.1] == b'*' { println!("{:?}", u); }
                        symbol_thing[u.0][u.1].push(part_number);
                    }
                    part_number_sum += part_number;
                    println!("{}", numbuf);
                    adjacent.clear();
                }
                numbuf.clear();
            }
        }
    }

    let mut gear_ratio_sum = 0;

    for y in 0..height {
        for x in 0..width {
            if input0[y][x] == b'*' && symbol_thing[y][x].len() == 2 {
                let multiple = symbol_thing[y][x][0] * symbol_thing[y][x][1];
                gear_ratio_sum += multiple;
                println!("{} {} {}", x, y, multiple);
            }
        }
    }

    println!("\n{}\n{}", part_number_sum, gear_ratio_sum);
    Ok(())
}
