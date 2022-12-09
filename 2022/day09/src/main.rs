fn distsq(a: [isize; 2], b: [isize; 2]) -> isize {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    dx * dx + dy * dy
}

// shit. awful.
fn new_pos(head: [isize; 2], tail: [isize; 2]) -> [isize; 2] {
    let mut asdf = [(0, [0; 2]); 9];
    let q = [-1, 0, 1];
    for x in 0..3 {
        for y in 0..3 {
            let thing = [tail[0] + q[x], tail[1] + q[y]];
            asdf[x * 3 + y] = (distsq(head, thing), thing);
        }
    }
    asdf.sort();
    asdf[0].1
}

fn hello(tails: usize) {
    let input0 = std::fs::read_to_string("input").unwrap();

    const SIZE: usize = 1000;
    let mut visited: [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];
    let mut pos = [[(SIZE / 2) as isize; 2]; 10];

    for line in input0.trim().split('\n') {
        let cmds: Vec<&str> = line.split(' ').collect();
        let steps = cmds[1].parse::<usize>().unwrap();
        for _ in 0..steps {
            let original_pos = pos;

            match cmds[0] {
                "U" => {
                    pos[0][1] -= 1;
                }
                "D" => {
                    pos[0][1] += 1;
                }
                "L" => {
                    pos[0][0] -= 1;
                }
                "R" => {
                    pos[0][0] += 1;
                }
                _ => (),
            }

            if tails == 1 {
                // This unfortunately didn't work with multiple tails...
                if distsq(pos[0], pos[1]) >= 4 {
                    pos[1] = original_pos[0];
                }
            } else {
                for i in 0..tails {
                    if distsq(pos[i], pos[i + 1]) >= 4 {
                        let newpos = new_pos(pos[i], pos[i + 1]);
                        // if newpos != pos[i+1] {
                        //     println!("{} - {:?} -> {:?}", cmds[0], pos[i+1], newpos);
                        // }
                        pos[i + 1] = newpos;
                    }
                }
            }

            visited[pos[tails][0] as usize][pos[tails][1] as usize] = true;

            // Print a pretty grid, which as useful for the part 2 test input...
            /*
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
            */
            /*
            println!("==========================");
            for y in 490..510 {
                for x in 488..515 {
                    let letters = b"H123456789";
                    let mut printed = false;
                    for i in 0..10 {
                        if pos[i] == [x, y] {
                            print!("{}", letters[i] as char);
                            printed = true;
                            break;
                        }
                    }
                    if !printed {
                        if [x as usize, y as usize] == [SIZE / 2, SIZE / 2] {
                            print!("s");
                        } else {
                            print!(".");
                        }
                    }
                }
                println!("");
            }
            */
        }
    }

    let answer: usize = visited
        .iter()
        .map(|v| v.iter().filter(|z| **z == true).count())
        .sum();

    println!("{}", answer);
}

fn main() {
    hello(1);
    hello(9);
}
