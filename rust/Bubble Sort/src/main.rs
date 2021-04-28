#![allow(unused_imports)]
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines().skip(1) {
        let mut nums: Vec<i32> = line?
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        let mut x: (u32, u32) = (0, 0);
        let mut sorted = false;
        while !sorted {
            sorted = true;
            for o in 0..nums.len() - 1 {
                if nums[o] > nums[o + 1] {
                    nums.swap(o, o + 1);
                    x.1 += 1;
                    sorted = false;
                }
            }
            x.0 += 1;
        }
        print!("{} {} ", x.0, x.1);
    }

    Ok(())
}
