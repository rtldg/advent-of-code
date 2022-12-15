use std::collections::*;
#[macro_use]
extern crate scan_fmt;

pub struct Thing2 {
    sx: i32,
    sy: i32,
    bx: i32,
    by: i32,
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    let mut things: Vec<(Thing2, i32)> = vec![];

    for line in input {
        let (sx, sy, bx, by) = scan_fmt!(
            line.trim(),
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i32,
            i32,
            i32,
            i32
        )
        .unwrap();
        things.push((
            Thing2 {
                sx: sx,
                sy: sy,
                bx: bx,
                by: by,
            },
            (sx.abs_diff(bx) + sy.abs_diff(by)) as i32,
        ));
    }

    let mut covered: HashSet<(i32, i32)> = HashSet::new();
    let target_row = 2_000_000;
    // let target_row = 10;

    for thing in &things {
        for a in 0..thing.1 + 1 {
            if target_row == thing.0.sy + a {
                let y = thing.0.sy + a;
                for b in 0..thing.1 + 1 - a {
                    covered.insert((thing.0.sx + b, y));
                    covered.insert((thing.0.sx - b, y));
                }
            }
            if target_row == thing.0.sy - a {
                let y = thing.0.sy - a;
                for b in 0..thing.1 + 1 - a {
                    covered.insert((thing.0.sx + b, y));
                    covered.insert((thing.0.sx - b, y));
                }
            }
        }
    }

    for thing in &things {
        covered.remove(&(thing.0.bx, thing.0.by));
    }

    println!("covered.len() = {}", covered.len());

    let distress = |x: i32, y: i32| {
        for thing in &things {
            if x.abs_diff(thing.0.sx) + y.abs_diff(thing.0.sy) <= thing.1 as u32 {
                return false;
            }
        }
        true
    };

    let range = 0..4_000_000;
    // let range = 0..20;

    for thing in &things {
        for a in 0..thing.1 + 1 {
            let b = thing.1 + 1 - a;
            let (x, y) = (thing.0.sx + b + 1, thing.0.sy + a + 1);
            if range.contains(&x) && range.contains(&y) && distress(x, y) {
                println!("{},{} = {}", x, y, x as isize * 4_000_000 + y as isize);
            }
            let (x, y) = (thing.0.sx - b - 1, thing.0.sy + a + 1);
            if range.contains(&x) && range.contains(&y) && distress(x, y) {
                println!("{},{} = {}", x, y, x as isize * 4_000_000 + y as isize);
            }
            let (x, y) = (thing.0.sx + b + 1, thing.0.sy + a - 1);
            if range.contains(&x) && range.contains(&y) && distress(x, y) {
                println!("{},{} = {}", x, y, x as isize * 4_000_000 + y as isize);
            }
            let (x, y) = (thing.0.sx - b - 1, thing.0.sy + a - 1);
            if range.contains(&x) && range.contains(&y) && distress(x, y) {
                println!("{},{} = {}", x, y, x as isize * 4_000_000 + y as isize);
            }
        }
    }
}
