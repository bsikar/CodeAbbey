#![allow(unused_imports)]
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines().skip(1) {
        let nums: Vec<f32> = line?
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        let x = nums[0] / nums[1].powf(2.);
        print!(
            "{} ",
            match x {
                _ if x < 18.5 => "under",
                _ if x < 25.0 => "normal",
                _ if x < 30.0 => "over",
                _ => "obese",
            }
        );
    }

    Ok(())
}
