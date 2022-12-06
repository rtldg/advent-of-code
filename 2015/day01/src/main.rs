fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let sum = s
        .chars()
        .fold(0, |acc, c| acc + if c == '(' { 1 } else { -1 });
    println!("{}", sum);
    let mut sum: i32 = 0;
    for (i, c) in s.chars().enumerate() {
        sum += if c == '(' { 1 } else { -1 };
        if sum == -1 {
            println!("{}", i + 1);
            break;
        }
    }
}
