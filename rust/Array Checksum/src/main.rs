#![allow(unused_imports)]
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines().skip(1) {
        print!(
            "{} ",
            line?
                .split_whitespace()
                .map(|i| i.parse::<usize>().unwrap())
                .fold(0, |x, i| ((x + i) * 113) % 10000007)
        );
    }

    Ok(())
}
