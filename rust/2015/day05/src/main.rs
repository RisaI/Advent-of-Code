use core::str;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("data.txt")?);

    let mut nice_a = 0;
    let mut nice_b = 0;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        if line
            .chars()
            .filter(|c| "aeiou".contains(*c))
            .take(3)
            .count()
            == 3
            && line.as_bytes().windows(2).any(|w| w[0] == w[1])
            && !line
                .as_bytes()
                .windows(2)
                .any(|w| matches!(&[w[0], w[1]], b"ab" | b"cd" | b"pq" | b"xy"))
        {
            nice_a += 1;
        }

        if line
            .as_bytes()
            .windows(2)
            .enumerate()
            .any(|(i, w)| line[(i + 2)..].contains(str::from_utf8(w).unwrap()))
            && line.as_bytes().windows(3).any(|w| w[0] == w[2])
        {
            nice_b += 1;
        }
    }

    println!("{nice_a} nice words (a)");
    println!("{nice_b} nice words (b)");

    Ok(())
}
