#![allow(unused_imports)]
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines().skip(1) {
        let count = line?.chars().filter(|&c| "aeiouy".contains(c)).count();
        print!("{} ", count);
    }

    Ok(())
}
