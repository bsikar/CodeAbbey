use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines() {
        let nums: Vec<u32> = line
            .unwrap()
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        println!("{}", nums[0] + nums[1]);
    }

    Ok(())
}
