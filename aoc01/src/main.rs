fn main() {
    let mut x: Vec<i64> = std::fs::read_to_string("input")
        .unwrap()
        .split("\n\n")
        .map(|y| y.split('\n').map(|z| z.parse::<i64>().unwrap_or(0)).sum())
        .collect();
    x.sort();
    println!(
        "a {} - b {}",
        x[x.len() - 1],
        x.iter().rev().take(3).sum::<i64>()
    );
}
