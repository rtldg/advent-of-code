#![feature(let_chains)]
#![allow(non_snake_case)]

use std::collections::*;

use pathfinding::prelude::astar;

fn neighbors(y: usize, x: usize, grid: &Vec<Vec<u8>>) -> Vec<((usize, usize), usize)> {
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
                let heuristic = |(y, x)| goal.0.abs_diff(y) + goal.1.abs_diff(x);

                let mut openSet = HashSet::new();
                openSet.insert((y, x));
                let mut cameFrom: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
                let mut gScore: HashMap<(usize, usize), usize> = HashMap::new();
                gScore.insert((y, x), 0);
                let mut fScore: HashMap<(usize, usize), usize> = HashMap::new();
                fScore.insert((y, x), heuristic((y, x)));

                let mut steps = 99999999;

                while openSet.len() > 0 {
                    let mut current = openSet
                        .iter()
                        .map(|val| (fScore[val], *val))
                        .min()
                        .unwrap()
                        .1;
                    if current == goal {
                        let mut total_path = HashSet::new();
                        while let Some(c) = cameFrom.get(&current) {
                            current = *c;
                            total_path.insert(current);
                        }
                        steps = total_path.len();
                        break;
                    }

                    openSet.remove(&current);
                    for neighbor in neighbors(current.0, current.1, &grid) {
                        let tentative_gScore = gScore[&current] + neighbor.1;
                        if tentative_gScore < *gScore.get(&neighbor.0).unwrap_or(&9999999) {
                            cameFrom.insert(neighbor.0, current);
                            gScore.insert(neighbor.0, tentative_gScore);
                            fScore.insert(neighbor.0, tentative_gScore + heuristic(neighbor.0));
                            openSet.insert(neighbor.0);
                        }
                    }
                }

                /*
                let steps = astar(
                    &(y, x),
                    |&(y, x)| neighbors(y, x, &grid),
                    |&(y, x)| goal.0.abs_diff(y) + goal.1.abs_diff(x),
                    |&p| p == goal,
                )
                .map(|r| r.1)
                .unwrap_or(9999999);
                */
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
