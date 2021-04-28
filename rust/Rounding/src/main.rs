#![allow(unused_imports)]
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines().skip(1) {
        let nums: Vec<f32> = line
            .unwrap()
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();

        print!("{} ", (nums[0] / nums[1]).round());
    }

    Ok(())
}
