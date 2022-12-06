fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let mut papersqft: usize = 0;
    let mut ribbonft: usize = 0;
    for line in s.split('\n') {
        let mut dims: Vec<usize> = line.split('x').map(|s| s.parse().unwrap()).collect();
        let a = 2 * dims[0] * dims[1];
        let b = 2 * dims[1] * dims[2];
        let c = 2 * dims[2] * dims[0];
        let smallest_dist = std::cmp::min(a, std::cmp::min(b, c)) / 2;
        papersqft += a + b + c + smallest_dist;
        dims.sort();
        ribbonft += (dims[0] * dims[1] * dims[2]) + (dims[0] + dims[1]) * 2;
    }
    println!("{}\n{}", papersqft, ribbonft);
}
