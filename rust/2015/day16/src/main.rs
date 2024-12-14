use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use winnow::{
    ascii::{alpha1, digit1},
    combinator::{preceded, separated, separated_pair},
    error::InputError,
    Parser,
};

fn objects_parser<'a>() -> impl Parser<&'a str, (String, usize), InputError<&'a str>> {
    separated_pair(alpha1::<&str, InputError<&str>>, ": ", digit1.parse_to())
        .map(|(name, quantity)| (name.to_string(), quantity))
}

fn main() -> anyhow::Result<()> {
    let detected: HashMap<String, usize> = separated(1.., objects_parser(), '\n')
        .parse(
            "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1",
        )
        .map_err(|e| anyhow::format_err!("{e}"))?;

    let reader = BufReader::new(File::open("data.txt")?);

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let (sue, objects): (usize, Vec<_>) = separated_pair(
            preceded("Sue ", digit1.parse_to::<usize>()),
            ": ",
            separated(1.., objects_parser(), ", "),
        )
        .parse(&line)
        .map_err(|e| anyhow::format_err!("{e}"))?;

        if objects.iter().all(|(tp, q)| detected.get(tp).unwrap() == q) {
            println!("Sue {sue} matches! (p1)");
        }

        if objects.iter().all(|(tp, q)| {
            let target = detected.get(tp).unwrap();

            match tp.as_str() {
                "cats" | "trees" => q > target,
                "pomeranians" | "goldfish" => q < target,
                _ => target == q,
            }
        }) {
            println!("Sue {sue} matches! (p2)");
        }
    }

    Ok(())
}
