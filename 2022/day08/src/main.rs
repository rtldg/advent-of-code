#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn hello(ispart2: bool) {
    let mut input0 = std::fs::read_to_string("input").unwrap();
    let mut answer = 0;

    let mut grid: Vec<Vec<u8>> = vec![];

    for line in input0.split('\n') {
        let mut v = vec![];
        for c in line.chars() {
            v.push(c as u8 - b'0');
        }
        grid.push(v);
    }

    let h = grid.len();
    let w = grid[0].len();
    println!("w = {} | h = {}", w, h);

    if ispart2 {
        let mut highestscore = 0;

        for y in 1..h-1 {
            for x in 1..w-1 {
                let tree = grid[y][x];
                let mut treescore = 0;

                for z in (0..x).rev() {
                    treescore += 1;
                    if grid[y][z] >= tree {
                        break;
                    }
                }

                let mut multi = 0;
                for z in x+1..w {
                    multi += 1;
                    if grid[y][z] >= tree {
                        break;
                    }
                }

                treescore *= multi;

                let mut multi = 0;
                for z in (0..y).rev() {
                    multi += 1;
                    if grid[z][x] >= tree {
                        break;
                    }
                }

                treescore *= multi;

                let mut multi = 0;
                for z in y+1..h {
                    multi += 1;
                    if grid[z][x] >= tree {
                        break;
                    }
                }

                treescore *= multi;

                highestscore = core::cmp::max(highestscore, treescore);
            }
        }

        println!("highestscore = {}", highestscore);

        return;
    }

    // exterior trees
    answer += (w - 1) * 2 + (h - 1) * 2;

    let mut counted: Vec<Vec<bool>> = vec![vec![false; w]; h];

    for x in 1..w-1 {
        let mut maxabove = grid[0][x];
        for y in 1..h-1 {
            let tree = grid[y][x];
            if tree > maxabove {
                maxabove = tree;
                if !counted[y][x] {
                    println!("counting x {} y {} -- {}", x, y, tree);
                    answer += 1;
                    counted[y][x] = true;
                }
            }
        }
    }

    for x in 1..w-1 {
        let mut maxbelow = grid[h-1][x];
        for y in (1..h-1).rev() {
            let tree = grid[y][x];
            if tree > maxbelow {
                maxbelow = tree;
                if !counted[y][x] {
                    println!("counting x {} y {} -- {}", x, y, tree);
                    answer += 1;
                    counted[y][x] = true;
                }
            }
        }
    }

    for y in 1..h-1 {
        let mut maxabove = grid[y][0];
        for x in 1..w-1 {
            let tree = grid[y][x];
            if tree > maxabove {
                maxabove = tree;
                if !counted[y][x] {
                    println!("counting x {} y {} -- {}", x, y, tree);
                    answer += 1;
                    counted[y][x] = true;
                }
            }
        }
    }

    for y in 1..h-1 {
        let mut maxbelow = grid[y][w-1];
        for x in (1..w-1).rev() {
            let tree = grid[y][x];
            if tree > maxbelow {
                maxbelow = tree;
                if !counted[y][x] {
                    println!("counting x {} y {} -- {}", x, y, tree);
                    answer += 1;
                    counted[y][x] = true;
                }
            }
        }
    }

    println!("\n{}", answer);
}

fn main() {
    hello(false);
    hello(true);
}
