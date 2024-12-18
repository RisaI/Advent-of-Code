use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;

fn parse_next_atom(input: &str) -> &str {
    if input.starts_with('e') {
        return &input[0..1];
    }

    if input
        .chars()
        .nth(1)
        .map(|c| c != 'e' && c.is_ascii_lowercase())
        == Some(true)
    {
        &input[0..2]
    } else {
        &input[0..1]
    }
}

fn parse_molecule(input: &str) -> impl Iterator<Item = &str> {
    (0..).scan(input, |state, _| {
        if state.is_empty() {
            return None;
        }

        let atom = parse_next_atom(state);
        *state = &state[atom.len()..];

        Some(atom)
    })
}

fn main() -> anyhow::Result<()> {
    let mut reader = BufReader::new(File::open("data.txt")?).lines();

    let mut atoms = HashMap::<String, u8>::default();
    let mut replacements = Vec::<(Vec<u8>, Vec<u8>)>::default();

    for line in reader.by_ref() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let (from, to) = line
            .split_once(" => ")
            .context("unknown replacement format")?;

        replacements.push((
            parse_molecule(from)
                .map(|atom| {
                    let next = atoms.len() as u8;
                    *atoms.entry(atom.to_string()).or_insert(next)
                })
                .collect(),
            parse_molecule(to)
                .map(|atom| {
                    let next = atoms.len() as u8;
                    *atoms.entry(atom.to_string()).or_insert(next)
                })
                .collect(),
        ));
    }

    let molecule = parse_molecule(&reader.next().context("unexpected end of input")??)
        .map(|atom| {
            let next = atoms.len() as u8;
            *atoms.entry(atom.to_string()).or_insert(next)
        })
        .collect::<Vec<_>>();

    // P1
    {
        let mut options = HashSet::new();

        for (idx, _) in molecule.iter().enumerate() {
            for (sub, by) in &replacements {
                if &molecule[idx..(idx + sub.len())] == sub {
                    options.insert(
                        molecule
                            .iter()
                            .take(idx)
                            .chain(by.iter())
                            .chain(molecule.iter().skip(idx + sub.len()))
                            .copied()
                            .collect::<Vec<_>>(),
                    );
                }
            }
        }

        println!("{} options", options.len());
    }

    // P2
    {
        replacements.sort_by_key(|(_, v)| v.len());
        replacements.reverse();

        fn inner(
            replacements: &[(Vec<u8>, Vec<u8>)],
            targets: &[Vec<u8>],
            current: &[u8],
            depth: usize,
            cap: &mut usize,
        ) {
            if depth >= *cap {
                return;
            }

            if targets.iter().any(|t| t == current) {
                *cap = depth;
                println!("{depth}");
                return;
            }

            for (by, sub) in replacements {
                if sub.len() > current.len() {
                    continue;
                }

                for i in 0..(current.len() - sub.len() + 1) {
                    if &current[i..(i + sub.len())] == sub {
                        let next = current
                            .iter()
                            .take(i)
                            .chain(by.iter())
                            .chain(current.iter().skip(i + sub.len()))
                            .copied()
                            .collect::<Vec<_>>();

                        inner(replacements, targets, &next, depth + 1, cap);
                    }
                }
            }
        }

        let targets = replacements
            .iter()
            .filter(|(k, _)| k == &vec![*atoms.get("e").unwrap()])
            .map(|(_, v)| v.clone())
            .collect::<Vec<_>>();

        let mut cap = 300;
        inner(&replacements, &targets, &molecule, 0, &mut cap);
        println!("{}", cap)
    }

    Ok(())
}
