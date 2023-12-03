#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = "test_input";
    //let input_file = "input";
    let input0 = std::fs::read_to_string(input_file)?;

    let input0: Vec<String> = std::fs::read_to_string(input_file)?
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect();

    for line in &input0 {
        if line == "" {
            continue;
        }
    }

    println!("{}\n{}", 0, 0);
    Ok(())
}
