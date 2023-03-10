fn main() {
    let input0 = std::fs::read_to_string("input").unwrap();
    const SIZE: usize = 800;
    let mut visited = vec![vec![0u16; SIZE]; SIZE];
    let mut pos = [[(SIZE / 2) as isize; 2]; 10];
    visited[SIZE / 2][SIZE / 2] = 0xFFFF;

    for line in input0.trim().split('\n') {
        let steps = line[2..].parse::<usize>().unwrap();
        for _ in 0..steps {
            match line.as_bytes()[0] {
                b'U' => pos[0][1] -= 1,
                b'D' => pos[0][1] += 1,
                b'L' => pos[0][0] -= 1,
                b'R' => pos[0][0] += 1,
                _ => (),
            }

            for i in 0..9 {
                /*
                [
                       .   [-1,-1][ 0,-1][ 1,-1]   .
                    [-1,-1]   .      .      .   [ 1,-1]
                    [-1, 0]   .      o      .   [ 1, 0]
                    [-1, 1]   .      .      .   [ 1, 1]
                       .   [-1, 1][ 0, 1][ 1, 1]   .
                ]
                */

                let dx = pos[i][0] - pos[i + 1][0];
                let dy = pos[i][1] - pos[i + 1][1];

                if dx.abs() > 1 || dy.abs() > 1 {
                    pos[i + 1][0] += dx.signum();
                    pos[i + 1][1] += dy.signum();
                    visited[pos[i + 1][0] as usize][pos[i + 1][1] as usize] |= 1 << (i + 1);
                }
            }

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

    let (mut answer1, mut answer2) = (0, 0);

    for a in visited.iter().flatten() {
        if (*a & (1 << 1)) != 0 {
            answer1 += 1;
        }
        if (*a & (1 << 9)) != 0 {
            answer2 += 1;
        }
    }

    println!("{}\n{}", answer1, answer2);
}
