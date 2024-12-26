use std::{collections::HashSet, iter::once};

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("input.txt")?;

    let deltas = data.chars().map(|ch| match ch {
        '^' => (0, 1),
        '>' => (1, 0),
        'v' => (0, -1),
        '<' => (-1, 0),
        _ => panic!("unknown symbol {ch}"),
    });

    let path = once((0, 0)).chain(deltas.clone().scan((0, 0), |pos, delta| {
        *pos = (pos.0 + delta.0, pos.1 + delta.1);

        Some(*pos)
    }));

    let positions = path.collect::<HashSet<_>>().len();

    println!("first year, santa visited {positions} houses");

    let next_year = once((0, 0))
        .chain(
            deltas
                .enumerate()
                .scan(((0, 0), (0, 0)), |(pos0, pos1), (i, delta)| {
                    let pos = if i % 2 == 0 { pos0 } else { pos1 };

                    *pos = (pos.0 + delta.0, pos.1 + delta.1);
                    Some(*pos)
                }),
        )
        .collect::<HashSet<_>>()
        .len();

    println!("second year, santas visited {next_year} houses");

    Ok(())
}
