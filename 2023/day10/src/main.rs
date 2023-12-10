#![feature(let_chains)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::cmp::Ordering;
use std::thread::current;
use std::{collections::HashMap, collections::HashSet, error::Error};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    let input_file = "test_input_p2";
    // let input_file = "input";
    let mut input0 = std::fs::read_to_string(input_file)?;
    let mut input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    let mut start = (0, 0);
    for (lineid, line) in input0.iter().enumerate() {
        if let Some(x) = line.find('S') {
            start = (x, lineid);
            break;
        }
    }

    let get_at = |pos: (usize, usize), offset: (isize, isize)| -> Option<(char, (usize, usize))> {
        let x = pos.0.overflowing_add_signed(offset.0);
        let y = pos.1.overflowing_add_signed(offset.1);
        if x.1 || y.1 || y.0 > input0.len() || x.0 > input0[0].len() {
            None
        } else {
            Some((input0[y.0].as_bytes()[x.0] as char, (x.0, y.0)))
        }
    };

    let get_connections =
        |(current_char, current_pos): (char, (usize, usize))| -> Vec<(char, (usize, usize))> {
            let mut connections = vec![];

            let connectors = [
                ('|', (0, -1), ['|', 'F', '7']),
                ('|', (0, 1), ['|', 'L', 'J']),
                ('-', (-1, 0), ['-', 'F', 'L']),
                ('-', (1, 0), ['-', 'J', '7']),
                ('L', (0, -1), ['|', 'F', '7']),
                ('L', (1, 0), ['-', 'J', '7']),
                ('J', (0, -1), ['|', 'F', '7']),
                ('J', (-1, 0), ['-', 'F', 'L']),
                ('7', (0, 1), ['|', 'L', 'J']),
                ('7', (-1, 0), ['-', 'F', 'L']),
                ('F', (0, 1), ['|', 'J', 'L']),
                ('F', (1, 0), ['-', 'J', '7']),
                ('S', (0, -1), ['|', '7', 'F']),
                ('S', (0, 1), ['|', 'J', 'L']),
                ('S', (-1, 0), ['-', 'F', 'L']),
                ('S', (1, 0), ['-', 'J', '7']),
            ];
            for connector in connectors {
                if current_char == connector.0 {
                    if let Some(v) = get_at(current_pos, connector.1)
                        && connector.2.contains(&v.0)
                    {
                        connections.push(v);
                    }
                }
            }

            connections
        };

    let mut farthest = 99999999;
    let starting_connections = get_connections(('S', start));
    for starting_connection in &starting_connections {
        let mut steps = 1;

        let mut pos = *starting_connection;
        let goal = if starting_connection.1 == starting_connections[0].1 {
            starting_connections[1].1
        } else {
            starting_connections[0].1
        };

        let mut visited = HashSet::from([start, pos.1]);
        while pos.1 != goal {
            steps += 1;
            // println!("{steps}");
            // let connections = get_connections(pos);

            for v in get_connections(pos) {
                if !visited.contains(&v.1) {
                    visited.insert(v.1);
                    pos = v;
                    println!("{:?}", pos);
                    break;
                }
            }
        }

        farthest = steps / 2 + 1;
    }

    println!("\n\n{:?}\n\n", farthest);

    Ok(())
}
