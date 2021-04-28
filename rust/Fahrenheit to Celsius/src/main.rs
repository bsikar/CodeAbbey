#![allow(unused_imports)]
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BufReader::new(File::open("input")?);

    for line in input.lines() {
        line?.split_whitespace().skip(1).for_each(|i| {
            let i: f32 = i.parse().unwrap();

            print!("{} ", ((i - 32.) * 5. / 9.).round())
        });
    }

    Ok(())
}
