use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use winnow::{
    ascii::{alpha1, digit1},
    PResult, Parser,
};

type Signal = u16;
type Wire = String;

#[derive(Debug)]
enum Operation<W> {
    Input(Signal),
    Copy(W),
    And(W, W),
    AndNum(Signal, W),
    Or(W, W),
    LShift(W, u8),
    RShift(W, u8),
    Not(W),
}

impl<T> Operation<T> {
    pub fn get_signal(&self, mut wiring: impl FnMut(&T) -> Option<Signal>) -> Option<Signal> {
        let v = match self {
            Operation::Input(v) => *v,
            Operation::Copy(a) => wiring(a)?,
            Operation::Not(a) => !wiring(a)?,
            Operation::And(a, b) => wiring(a)? & wiring(b)?,
            Operation::AndNum(a, b) => *a & wiring(b)?,
            Operation::Or(a, b) => wiring(a)? | wiring(b)?,
            Operation::LShift(a, b) => wiring(a)? << *b,
            Operation::RShift(a, b) => wiring(a)? >> *b,
        };

        Some(v)
    }

    pub fn dependencies_met(&self, wiring: impl Fn(&T) -> bool) -> bool {
        match self {
            Operation::Input(_) => true,
            Operation::Copy(a) => wiring(a),
            Operation::Not(a) => wiring(a),
            Operation::And(a, b) => wiring(a) && wiring(b),
            Operation::AndNum(_, b) => wiring(b),
            Operation::Or(a, b) => wiring(a) && wiring(b),
            Operation::LShift(a, _) => wiring(a),
            Operation::RShift(a, _) => wiring(a),
        }
    }
}

fn parse_wire(input: &mut &str) -> PResult<Wire> {
    alpha1
        .verify(|v: &str| v.len() <= 2)
        .map(|v: &str| v.to_string())
        .parse_next(input)
}

fn parse_operation(input: &mut &str) -> PResult<Operation<Wire>> {
    winnow::combinator::alt((
        (parse_wire, " AND ", parse_wire).map(|(a, _, b)| Operation::And(a, b)),
        (digit1.parse_to(), " AND ", parse_wire).map(|(a, _, b)| Operation::AndNum(a, b)),
        (parse_wire, " OR ", parse_wire).map(|(a, _, b)| Operation::Or(a, b)),
        ("NOT ", parse_wire).map(|(_, a)| Operation::Not(a)),
        (parse_wire, " LSHIFT ", digit1.parse_to()).map(|(a, _, s)| Operation::LShift(a, s)),
        (parse_wire, " RSHIFT ", digit1.parse_to()).map(|(a, _, s)| Operation::RShift(a, s)),
        (digit1.parse_to().map(Operation::Input)),
        (parse_wire).map(Operation::Copy),
    ))
    .parse_next(input)
}

fn resolve_wiring(wiring: &HashMap<Wire, Operation<Wire>>) -> HashMap<&Wire, Signal> {
    let mut known_signal = HashMap::<&Wire, Signal>::default();
    let mut unresolved_wires = wiring.keys().collect::<Vec<_>>();

    while !unresolved_wires.is_empty() {
        unresolved_wires.retain(|wire| {
            let op = wiring.get(*wire).unwrap();

            if op.dependencies_met(|w| known_signal.contains_key(w)) {
                let v = op.get_signal(|w| known_signal.get(w).copied()).unwrap();

                known_signal.insert(*wire, v);

                false
            } else {
                true
            }
        });
    }

    known_signal
}

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("data.txt")?);

    let start = std::time::Instant::now();
    let mut wiring = HashMap::<Wire, Operation<Wire>>::default();

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let (op, _, output) = (parse_operation, " -> ", parse_wire)
            .parse(&line)
            .map_err(|e| anyhow::format_err!("{e}"))?;

        wiring.insert(output, op);
    }

    let a_val = resolve_wiring(&wiring)
        .get(&String::from("a"))
        .copied()
        .unwrap();

    println!("a: {a_val}");

    wiring.remove("b");
    wiring.insert(String::from("b"), Operation::Input(a_val));

    let a_val = resolve_wiring(&wiring)
        .get(&String::from("a"))
        .copied()
        .unwrap();

    println!("a (p2): {a_val}");

    println!("finished in {}us", start.elapsed().as_micros());

    Ok(())
}
