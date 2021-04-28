#![allow(unused_imports)]
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines() {
        let mut nums: Vec<i32> = line?
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        nums.sort();
        println!("{} {}", nums.last().unwrap(), nums.first().unwrap());
    }

    Ok(())
}
