#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::error::Error;

fn is_symbol(c: char) -> bool {
    match c {
        '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '\\' | '/' | '+' | '-' | '='  => true,
        _ => false,
    }
}

fn is_adjacent(map: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
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
        if is_symbol(map[(y as isize + thing.1) as usize][(x as isize + thing.0) as usize] as char) {
            return true;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    // let input_file = "input";
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

    let mut part_number_sum = 0;

    for y in 0..height {
        let mut numbuf = String::new();
        let mut adjacent = false;

        for x in 0..width {
            let c = input0[y][x] as char;
            if c.is_ascii_digit() {
                adjacent = adjacent || is_adjacent(&input0, x, y);
                numbuf.push(c);
            } else {
                if adjacent {
                    // input0[y][x - numbuf.len()-1..x-1].fill(0);
                    part_number_sum += numbuf.parse::<usize>().unwrap();
                    println!("{}", numbuf);
                    adjacent = false;
                }
                numbuf.clear();
            }
        }
        /*
        if adjacent {
            input0[y][width - numbuf.len()..].fill(0);
            part_number_sum += numbuf.parse::<usize>().unwrap();
            println!("{}", numbuf);
        }
        */
    }


    let mut gear_ratio_sum = 0;




    println!("\n{}\n{}", part_number_sum, 0);
    Ok(())
}
