use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::{bail, Context};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    pub fn apply(self, a: bool, b: bool) -> bool {
        match self {
            Op::And => a && b,
            Op::Or => a || b,
            Op::Xor => a != b,
        }
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => bail!("unknown op '{s}'"),
        })
    }
}

type Wiring = HashMap<String, bool>;
type Gates = HashMap<String, (Op, [String; 2])>;

fn resolve_gates(mut known_wires: Wiring, gates: &Gates) -> Wiring {
    loop {
        let mut resolved_any = false;

        for (out, (op, [a, b])) in gates {
            if known_wires.contains_key(out) {
                continue;
            }

            if let [Some(a), Some(b)] = [known_wires.get(a), known_wires.get(b)] {
                known_wires.insert(out.to_string(), op.apply(*a, *b));
                resolved_any = true;
            }
        }

        if !resolved_any {
            break;
        }
    }

    assert!(
        gates.keys().all(|k| known_wires.contains_key(k)),
        "gates should all be resolved"
    );

    known_wires
}

fn unordered_eq<T: AsRef<str>, TT: AsRef<str>>(a: &[T], b: &[TT]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    (a[0].as_ref() == b[0].as_ref() && a[1].as_ref() == b[1].as_ref())
        || (a[0].as_ref() == b[1].as_ref() && a[1].as_ref() == b[0].as_ref())
}

fn z_value(wiring: &Wiring) -> usize {
    wiring
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .map(|(k, v)| {
            if *v {
                let by = k[1..].parse::<usize>().unwrap();
                1 << by
            } else {
                0
            }
        })
        .sum::<usize>()
}

fn main() -> anyhow::Result<()> {
    let mut reader = BufReader::new(File::open("input.txt")?).lines();

    let mut known_wires = HashMap::new();
    let mut gates = HashMap::<String, (Op, [String; 2])>::new();

    // Known inputs
    for line in reader.by_ref() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let Some((name, val)) = line.split_once(": ") else {
            bail!("unknown format: '{line}'");
        };

        known_wires.insert(name.to_string(), val == "1");
    }

    // Gates
    for line in reader {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let Some((op, target)) = line.split_once(" -> ") else {
            bail!("unknown format: '{line}'");
        };

        let mut split = op.split(' ');
        let [Some(a), Some(op), Some(b), None] = std::array::from_fn(|_| split.next()) else {
            bail!("unknown syntax: '{op}'");
        };

        gates.insert(target.to_string(), (op.parse()?, [a, b].map(String::from)));
    }

    // P1
    println!(
        "the result of the circuit = {}",
        z_value(&resolve_gates(known_wires, &gates))
    );

    // P2
    {
        macro_rules! finder {
            ($op:expr, $inputs:expr) => {
                |(k, (o, i)): (&String, &(Op, [String; 2]))| {
                    Some(k).filter(|_| *o == $op && unordered_eq(i, $inputs))
                }
            };
        }

        let mut found = HashSet::new();

        for i in 2..45 {
            let inputs = [format!("x{i:02}"), format!("y{i:02}")];

            let current_digit = gates
                .iter()
                .find_map(finder!(Op::Xor, &inputs))
                .with_context(|| format!("input not found for {i}"))?;

            let output = &gates.get(&format!("z{i:02}")).unwrap();

            if output.0 != Op::Xor {
                found.insert(format!("z{i:02}"));

                let adder = gates
                    .iter()
                    .find_map(|(k, (o, i))| {
                        Some(k).filter(|_| *o == Op::Xor && (i.contains(current_digit)))
                    })
                    .unwrap();

                found.insert(adder.to_string());
                continue;
            }

            if !output.1.contains(current_digit) {
                found.insert(current_digit.to_string());

                if let Some(v) = output.1.iter().find(|k| gates.get(*k).unwrap().0 != Op::Or) {
                    found.insert(v.to_string());
                }

                continue;
            }
        }

        let mut found = found.into_iter().collect::<Vec<_>>();
        found.sort();

        println!("{}", found.join(","));
    }

    Ok(())
}
