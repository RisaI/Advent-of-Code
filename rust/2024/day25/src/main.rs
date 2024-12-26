fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("data.txt")?;

    let (keys, locks) =
        data.split("\n\n")
            .fold((vec![], vec![]), |(mut keys, mut locks), schema| {
                let schema = schema
                    .trim()
                    .split('\n')
                    .map(|v| v.as_bytes())
                    .collect::<Vec<_>>();

                if schema[0] == b"#####" {
                    locks.push(std::array::from_fn::<usize, 5, _>(|i| {
                        schema.iter().position(|v| v[i] == b'.').unwrap() - 1
                    }));
                } else {
                    keys.push(std::array::from_fn::<usize, 5, _>(|i| {
                        schema.iter().rev().position(|v| v[i] == b'.').unwrap() - 1
                    }));
                }

                (keys, locks)
            });

    let combos = keys
        .iter()
        .map(|k| {
            locks
                .iter()
                .filter(|l| k.iter().zip(l.iter()).all(|(&k, &l)| k + l <= 5))
                .count()
        })
        .sum::<usize>();

    println!("{} unique combos", combos);

    Ok(())
}
