use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines().skip(1) {
        let nums: Vec<u32> = line?
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        println!("{}", nums.iter().sum::<u32>());
    }

    Ok(())
}
