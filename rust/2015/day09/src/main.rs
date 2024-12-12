use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::bail;
use itertools::Itertools;
use winnow::{
    ascii::{alpha1, digit1},
    combinator::separated_pair,
    error::InputError,
    Parser,
};

fn parse_line<'a>() -> impl Parser<&'a str, ((String, String), usize), InputError<&'a str>> {
    separated_pair(
        separated_pair(alpha1.parse_to(), " to ", alpha1.parse_to()),
        " = ",
        digit1.parse_to::<usize>(),
    )
}

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("data.txt")?);

    let mut places = HashMap::<String, usize>::default();
    let mut distances = HashMap::<(usize, usize), usize>::default();

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let mut parser = parse_line();

        let Ok(((a, b), dist)) = parser.parse(line.as_str()) else {
            bail!("malformed input: {line}");
        };

        let [a, b] = [a, b].map(|v| {
            let idx = places.len();
            *places.entry(v).or_insert(idx)
        });

        distances.insert((a, b), dist);
        distances.insert((b, a), dist);
    }

    let (min, max) = places
        .values()
        .copied()
        .permutations(places.len())
        .map(|p| {
            p.windows(2)
                .map(|w| distances.get(&(w[0], w[1])).unwrap())
                .sum::<usize>()
        })
        .fold((usize::MAX, usize::MIN), |(min, max), v| {
            (min.min(v), max.max(v))
        });

    println!("shortest path = {min}");
    println!("longest path = {max}");

    Ok(())
}
