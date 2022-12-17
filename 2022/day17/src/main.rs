use std::collections::*;

fn main() {
    let tetrominoes: [[u8; 4]; 5] = [
        [0b00000, 0000000, 0b00000, 0b11110],
        [0b00000, 0b01000, 0b11100, 0b01000],
        [0b00000, 0b00100, 0b00100, 0b11100],
        [0b10000, 0b10000, 0b10000, 0b10000],
        [0b00000, 0b00000, 0b11000, 0b11000],
    ];

    let input = include_str!("../input").trim().as_bytes();
    // using a (reverse) stack would probably be simpler...
    let mut tower = VecDeque::<u8>::new();
    tower.push_front(0xFF);

    let mut jets_moved = 0;
    // for pieces in 0..1000000000000 {
    for pieces in 0..2022 {
        let mut pos = 3;
        let mut tetromino = tetrominoes[pieces % 5];
        for _ in 0..7 {
            tower.push_front(Default::default());
        }
        'move_piece: loop {
            let mut try_jet = || {
                let (border, rotations_right) = if input[jets_moved % input.len()] == b'<' {
                    (0b_1_00000_0, 7)
                } else {
                    (0b_0_00000_1, 1)
                };

                for row in tetromino {
                    if (row & border) != 0 {
                        return;
                    }
                }

                let mut piece = tetromino;
                for row in piece.iter_mut() {
                    *row = row.rotate_right(rotations_right);
                }

                for (i, row) in piece.iter().enumerate() {
                    // println!("len = {}, pos = {}, i = {}", tower.len(), pos, i);
                    let trow = tower[pos - 3 + i];
                    if (row | trow).count_ones() != (row.count_ones() + trow.count_ones()) {
                        return;
                    }
                }

                tetromino = piece;
            };

            try_jet();
            jets_moved += 1;

            // let lcm = input.len() * 5;
            // if (lcm % jets_moved) == 0 {
            //     println!("hey at piece {}", pieces);
            // }

            // check if blocked now...
            for i in 0..2 {
                if (tower[pos + 1 - i] | tetromino[3 - i]).count_ones()
                    != (tower[pos + 1 - i].count_ones() + tetromino[3 - i].count_ones())
                {
                    for (i, row) in tetromino.iter().enumerate() {
                        tower[pos - 3 + i] |= *row;
                    }
                    break 'move_piece;
                }
            }
            pos += 1;
        }
        // ez cleanup lol
        while tower[0] == 0 {
            tower.pop_front();
        }
        // if pieces == 8595 {
        //     println!("{} = {} - 1 + {}", tower.len()-1+rows_removed, tower.len(), rows_removed);
        // }
    }

    let height = tower.len() - 1;

    println!("height = {}", height);
}
