fn main() {
    let mut x: Vec<i64> = std::fs::read_to_string("input")
        .unwrap()
        .split("\n\n")
        .map(|y| y.split('\n').map(|z| z.parse::<i64>().unwrap_or(0)).sum())
        .collect();
    x.sort();
    x.reverse();
    println!("a {} - b {}", x[0], x[0] + x[1] + x[2]);
}
