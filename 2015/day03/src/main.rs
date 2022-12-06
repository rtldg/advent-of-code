fn hello(ispart2: bool) {
    let s = std::fs::read_to_string("input").unwrap();
    let mut ohgod: Vec<Vec<u8>> = vec![vec![0; 300]; 300]; // thank god 300x300 & 150x150 was enough for this "infinite two-dimensional grid of houses"
    let mut pos: [(usize, usize); 2] = [(150, 150), (150, 150)];
    ohgod[150][150] = 2;
    let mut who: usize = 0;
    for c in s.chars() {
        match c {
            '>' => pos[who].0 += 1,
            '<' => pos[who].0 -= 1,
            '^' => pos[who].1 -= 1,
            'v' => pos[who].1 += 1,
            _ => {}
        }
        ohgod[pos[who].0][pos[who].1] += 1;
        if ispart2 {
            who ^= 1;
        }
    }
    let sum: usize = ohgod
        .iter()
        .map(|v| v.iter().filter(|a| **a > 0).count())
        .sum();
    println!("{}", sum);
}

fn main() {
    hello(false);
    hello(true);
}
