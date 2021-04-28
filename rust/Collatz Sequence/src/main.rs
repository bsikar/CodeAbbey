#![allow(unused_imports)]
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines().skip(1) {
        line?
            .split_whitespace()
            .map(|i| i.parse::<usize>().unwrap())
            .for_each(|mut x| {
                let mut y = 0;
                while x != 1 {
                    if x % 2 == 0 {
                        x = x / 2;
                    } else {
                        x = 3 * x + 1;
                    }
                    y += 1;
                }
                print!("{} ", y);
            });
    }

    Ok(())
}
