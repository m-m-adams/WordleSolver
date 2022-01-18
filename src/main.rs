use anyhow::{bail, Result};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
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
    let mut wordle = Wordle::new(words.to_vec(), "aback");
    let first_guess = wordle.guess();
    //let now = Instant::now();

    for w in words.clone() {
        let mut wordle = Wordle::new(words.to_vec(), &w);
        let (f, guess) = wordle.solve(first_guess)?;
        if f != w {
            bail!("failed to solve {w}, got {f}");
        }

        println!("{f}, {guess}");
    }

    //println!("took {} ms", now.elapsed().as_millis());

    Ok(())
}
