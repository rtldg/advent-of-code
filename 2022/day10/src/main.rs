#![allow(non_snake_case)]

fn calc(adds: &mut Vec<(isize, isize)>, clock: &mut isize, X: &mut isize) {
    *clock += 1;
    adds.retain(|elem| {
        if elem.0 > *clock {
            true
        } else {
            *X += elem.1;
            false
        }
    });
    let pixel = *clock % 40 - 1;
    if pixel == 0 {
        println!("");
    }
    if pixel.abs_diff(*X) < 2 && *X >= 0 && *X <= 40 {
        print!("#");
    } else {
        print!(".");
    }
}

fn signal(clock: isize, X: isize) -> isize {
    match clock {
        20 | 60 | 100 | 140 | 180 | 220 => X * clock,
        _ => 0,
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut answer = 0;

    let mut X: isize = 1;
    let mut clock: isize = 0;
    let mut adds: Vec<(isize, isize)> = vec![];

    for line in input.trim().split('\n') {
        let cmds: Vec<&str> = line.split(' ').collect();
        calc(&mut adds, &mut clock, &mut X);
        answer += signal(clock, X);

        if cmds[0] == "addx" {
            let V = cmds[1].parse::<isize>().unwrap();
            adds.push((clock + 2, V));
            calc(&mut adds, &mut clock, &mut X);
            answer += signal(clock, X);
        }
    }

    println!("\n\n{}", answer);
}
