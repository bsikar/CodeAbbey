#![allow(unused_imports)]
use std::{
    cmp,
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines().skip(1) {
        let nums: Vec<u32> = line?
            .split_whitespace()
            .map(|i| {
                let mut x = 0;
                i.chars()
                    .map(|c| {
                        x += 1;
                        c.to_digit(10).unwrap() * x
                    })
                    .sum::<u32>()
            })
            .collect();
        for i in nums.iter() {
            print!("{} ", i);
        }
    }

    Ok(())
}
