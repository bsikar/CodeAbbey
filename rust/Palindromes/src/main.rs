#![allow(unused_imports)]
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines().skip(1) {
        let word: String = line?
            .chars()
            .filter(|&c| c.is_ascii_alphabetic())
            .collect::<String>()
            .to_lowercase();
        print!(
            "{} ",
            if word == word.chars().rev().collect::<String>() {
                "Y"
            } else {
                "N"
            }
        );
    }

    Ok(())
}
