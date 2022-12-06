fn char_to_priority(c: char) -> u32 {
    let c = c as u8;
    let x = if c >= b'a' && c <= b'z' {
        c - b'a' + 1
    } else {
        c - b'A' + 1 + 26
    };
    x as u32
}

fn part1() {
    let mut summy: u32 = 0;

    for line in std::fs::read_to_string("input").unwrap().split('\n') {
        let a = &line[0..line.len() / 2];
        let b = &line[line.len() / 2..];

        for c in b.chars() {
            if let Some(_) = a.find(c) {
                summy += char_to_priority(c);
                break;
            }
        }
    }
    println!("{}", summy);
}

fn part2() {
    let mut summy: u32 = 0;

    for chunk in std::fs::read_to_string("input")
        .unwrap()
        .split('\n')
        .collect::<Vec<&str>>()
        .chunks(3)
    {
        for c in chunk[0].chars() {
            if let Some(_) = chunk[1].find(c) {
                if let Some(_) = chunk[2].find(c) {
                    summy += char_to_priority(c);
                    break;
                }
            }
        }
    }
    println!("{}", summy);
}

fn main() {
    part1();
    part2();
}
