#![allow(unused_imports)]
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = BufReader::new(File::open("input")?);
    let mut line = String::new();
    input.read_line(&mut line)?;

    let mut x = vec![
        0;
        line.split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap()
    ];

    for line in input.lines() {
        line?
            .split_whitespace()
            .for_each(|i| x[i.parse::<usize>().unwrap() - 1] += 1);
    }

    for i in x.iter() {
        print!("{} ", i);
    }

    Ok(())
}
