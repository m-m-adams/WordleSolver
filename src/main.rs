use anyhow::Result;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str,
    time::Instant,
};

use wordle::Wordle;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() -> Result<()> {
    let words = lines_from_file("wordlist.txt");

    let mut wordle = Wordle::new(words[0..500].to_vec(), "affix");
    let now = Instant::now();
    let f = &wordle.guess();
    let first = str::from_utf8(f)?;

    println!("{}", now.elapsed().as_micros());
    println!("first guess is {first}");
    Ok(())
}
