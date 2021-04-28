#![allow(unused_imports)]
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines().skip(1) {
        let nums: Vec<i32> = line?
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();

        print!(
            "{} ",
            if (nums[0] + nums[1]) <= nums[2]
                || (nums[1] + nums[2]) <= nums[0]
                || (nums[2] + nums[0]) <= nums[1]
            {
                "0"
            } else {
                "1"
            }
        );
    }

    Ok(())
}
