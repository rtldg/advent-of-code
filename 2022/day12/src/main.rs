#![feature(let_chains)]

use pathfinding::prelude::astar;

fn get_paths(y: usize, x: usize, grid: &Vec<Vec<u8>>) -> Vec<((usize, usize), usize)> {
    let myheight = grid[y][x];
    let mut v = vec![];

    if let Some(h) = grid.get(y + 1).and_then(|g| g.get(x)) {
        if myheight > *h || h.abs_diff(myheight) <= 1 {
            v.push(((y + 1, x), 1));
        }
    }
    if y > 0 && let Some(h) = grid.get(y - 1).and_then(|g| g.get(x)) {
        if myheight > *h || h.abs_diff(myheight) <= 1 {
            v.push(((y - 1, x), 1));
        }
    }
    if let Some(h) = grid.get(y).and_then(|g| g.get(x + 1)) {
        if myheight > *h || h.abs_diff(myheight) <= 1 {
            v.push(((y, x + 1), 1));
        }
    }
    if x > 0 && let Some(h) = grid.get(y).and_then(|g| g.get(x - 1)) {
        if myheight > *h || h.abs_diff(myheight) <= 1 {
            v.push(((y, x - 1), 1));
        }
    }

    v
}

fn main() {
    let input0: Vec<String> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    let mut grid: Vec<Vec<u8>> = vec![];
    let mut start = (0, 0);
    let mut goal = (0, 0);

    for line in input0 {
        let mut v: Vec<u8> = vec![];
        for (i, c) in line.chars().enumerate() {
            let h = match c {
                'S' => {
                    start = (grid.len(), i);
                    0
                }
                'E' => {
                    goal = (grid.len(), i);
                    25
                }
                _ => c as u8 - b'a',
            };
            v.push(h)
        }
        grid.push(v);
    }

    println!("start = {:?} goal = {:?}", start, goal);

    let mut vsteps: Vec<usize> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                let steps = astar(
                    &(y, x),
                    |&(y, x)| get_paths(y, x, &grid),
                    |&(y, x)| goal.0.abs_diff(y) + goal.1.abs_diff(x),
                    |&p| p == goal,
                )
                .map(|r| r.1)
                .unwrap_or(9999999);
                if (y, x) == start {
                    println!("{}", steps);
                }
                vsteps.push(steps);
            }
        }
    }
    vsteps.sort();
    println!("{}", vsteps[0]);
}
