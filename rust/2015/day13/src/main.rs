use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter::once,
};

use itertools::Itertools;
use winnow::{
    ascii::{alpha1, digit1},
    combinator::{alt, separated_pair},
    error::InputError,
    Parser,
};

fn line_parser<'a>() -> impl Parser<&'a str, (String, String, isize), InputError<&'a str>> {
    (
        alpha1,
        " would ",
        separated_pair(alt(("gain", "lose")), ' ', digit1.parse_to::<isize>()),
        " happiness units by sitting next to ",
        alpha1,
        '.',
    )
        .map(
            |(a, _, (sign, val), _, b, _): (&str, &str, (&str, isize), &str, &str, char)| {
                (
                    a.to_string(),
                    b.to_string(),
                    val * match sign {
                        "gain" => 1,
                        "lose" => -1,
                        _ => panic!("unknown sign"),
                    },
                )
            },
        )
}

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    let mut names = HashSet::<String>::default();
    let mut relationships = HashMap::<(String, String), isize>::default();

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let mut parser = line_parser();

        let (a, b, delta) = parser
            .parse(&line)
            .map_err(|e| anyhow::format_err!("{e}"))?;

        let (a, b) = if a <= b { (a, b) } else { (b, a) };

        names.extend([a.clone(), b.clone()]);
        *relationships.entry((a, b)).or_default() += delta;
    }

    for names in [
        names
            .iter()
            .cloned()
            .chain(once(String::from("Me")))
            .collect(),
        names,
    ]
    .into_iter()
    .rev()
    {
        let max_happiness = names
            .iter()
            .permutations(names.len())
            .map(|permutation| {
                let last = [*permutation.last().unwrap(), *permutation.first().unwrap()];

                permutation
                    .windows(2)
                    .chain(once(last.as_slice()))
                    .map(|pair| {
                        let [a, b] = if pair[0] <= pair[1] {
                            [pair[0], pair[1]]
                        } else {
                            [pair[1], pair[0]]
                        };

                        relationships
                            .get(&(a.into(), b.into()))
                            .copied()
                            .unwrap_or_default()
                    })
                    .sum::<isize>()
            })
            .max()
            .unwrap();

        println!("max happiness = {max_happiness}");
    }

    Ok(())
}
