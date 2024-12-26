use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::bail;
use winnow::{
    ascii::digit1,
    combinator::{empty, separated_pair},
    error::InputError,
    Parser,
};

const ROWS: usize = 1_000;
const COLS: usize = 1_000;

type Point = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Op {
    On,
    Off,
    Toggle,
}

impl Op {
    pub fn apply(self, val: bool) -> bool {
        match self {
            Op::On => true,
            Op::Off => false,
            Op::Toggle => !val,
        }
    }

    pub fn apply_alt(self, val: usize) -> usize {
        match self {
            Op::On => val + 1,
            Op::Toggle => val + 2,
            Op::Off => val.saturating_sub(1),
        }
    }

    pub fn apply_to_range(self, data: &mut [bool], from: Point, to: Point) {
        for y in from.1..=to.1 {
            for x in from.0..=to.0 {
                let idx = x + COLS * y;
                data[idx] = self.apply(data[idx]);
            }
        }
    }
    pub fn apply_alt_to_range(self, data: &mut [usize], from: Point, to: Point) {
        for y in from.1..=to.1 {
            for x in from.0..=to.0 {
                let idx = x + COLS * y;
                data[idx] = self.apply_alt(data[idx]);
            }
        }
    }
}

fn parse_point<'a>() -> impl Parser<&'a str, (usize, usize), InputError<&'a str>> {
    separated_pair(digit1.parse_to(), ',', digit1.parse_to())
}

fn parse_line(mut input: &str) -> anyhow::Result<(Op, Point, Point)> {
    let Ok(((_, op), p1, _, p2)) = (
        winnow::combinator::alt((
            ("toggle ", empty.map(|_| Op::Toggle)),
            ("turn on ", empty.map(|_| Op::On)),
            ("turn off ", empty.map(|_| Op::Off)),
        )),
        parse_point(),
        " through ",
        parse_point(),
    )
        .parse_next(&mut input)
    else {
        bail!("failed to parse")
    };

    Ok((op, p1, p2))
}

fn main() -> anyhow::Result<()> {
    let mut field = vec![false; ROWS * COLS];
    let mut field_alt = vec![0usize; ROWS * COLS];
    let reader = BufReader::new(File::open("input.txt")?);

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let (op, p1, p2) = parse_line(&line)?;

        op.apply_to_range(&mut field, p1, p2);
        op.apply_alt_to_range(&mut field_alt, p1, p2);
    }

    println!("{} turned on", field.iter().filter(|v| **v).count());
    println!("{} total brightness", field_alt.iter().sum::<usize>());

    Ok(())
}
