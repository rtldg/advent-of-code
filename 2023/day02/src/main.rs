#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::cmp::max;

fn main() {
    let input0: Vec<String> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    let (max_red, max_green, max_blue) = (12, 13, 14);

    let mut possible_games: Vec<usize> = vec![];

    for (i, line) in input0.iter().enumerate() {
        if line == "" {
            continue;
        }

        let game: Vec<Vec<&str>> = line
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .split(';')
            .map(|s| s.trim().split(',').map(|ss| ss.trim()).collect())
            .collect();
        // println!("{:?}", game);

        let mut okay = true;
        for set in game {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            // println!("new set");
            for count_and_color in set {
                let count = count_and_color
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                // println!("{count_and_color} {count}");
                if count_and_color.contains("red") {
                    red = count;
                } else if count_and_color.contains("green") {
                    green = count;
                } else if count_and_color.contains("blue") {
                    blue = count;
                }
            }
            if red > max_red || green > max_green || blue > max_blue {
                // println!("{red} {green} {blue}");
                okay = false;
                break;
            }
        }

        if okay {
            // println!("adding {}", i+1);
            possible_games.push(i + 1);
        }
    }

    let mut answer2_sum = 0;
    for (i, line) in input0.iter().enumerate() {
        if line == "" {
            continue;
        }

        let game: Vec<Vec<&str>> = line
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .split(';')
            .map(|s| s.trim().split(',').map(|ss| ss.trim()).collect())
            .collect();

        let (mut red, mut green, mut blue) = (0, 0, 0);
        for set in game {
            for count_and_color in set {
                let count = count_and_color
                    .split(' ')
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                if count_and_color.contains("red") {
                    red = max(red, count);
                } else if count_and_color.contains("green") {
                    green = max(green, count);
                } else if count_and_color.contains("blue") {
                    blue = max(blue, count);
                }
            }
        }
        answer2_sum += red * green * blue;
    }

    println!("{}\n{}", possible_games.iter().sum::<usize>(), answer2_sum);
}
