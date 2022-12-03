fn give_score(theirs: u8, mine: u8) -> u8 {
    let table2: [u8; 5] = [0, 6, 3, 0, 6];
    mine + 1 + table2[(theirs + 2 - mine) as u8 as usize]
}

fn pick_hand(theirs: u8, outcome: u8) -> u8 {
    let table: [[u8; 3]; 3] = [[2, 0, 1], [0, 1, 2], [1, 2, 0]];
    table[outcome as usize][theirs as usize]
}

fn main() {
    let (mut a, mut b) = (0, 0);
    for x in std::fs::read_to_string("input").unwrap().split('\n') {
        let theirs = x.as_bytes()[0] - b'A';
        let mine = x.as_bytes()[2] - b'X';
        a += give_score(theirs, mine) as u32;
        b += give_score(theirs, pick_hand(theirs, mine)) as u32;
    }
    println!("{} {}", a, b);
}
