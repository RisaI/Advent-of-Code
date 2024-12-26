use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rustc_hash::FxHashMap as HashMap;

fn possible_combinations(
    combo: &str,
    towels: &[impl AsRef<str>],
    cache: &mut HashMap<String, usize>,
) -> usize {
    if combo.is_empty() {
        return 1;
    }

    if let Some(v) = cache.get(combo) {
        return *v;
    }

    let count = towels
        .iter()
        .map(|t| {
            if combo.starts_with(t.as_ref()) {
                possible_combinations(&combo[t.as_ref().len()..], towels, cache)
            } else {
                0
            }
        })
        .sum::<usize>();

    cache.insert(combo.into(), count);

    count
}

fn main() -> anyhow::Result<()> {
    let mut reader = BufReader::new(File::open("input.txt")?).lines();

    let towels = reader
        .by_ref()
        .take_while(|v| v.is_err() || !v.as_ref().unwrap().is_empty())
        .try_fold(vec![], |mut state, line| -> anyhow::Result<_> {
            state.extend(line?.split(", ").map(String::from));
            Ok(state)
        })?;

    let elapsed = std::time::Instant::now();

    let (_, possible_count, total_possible_combinations) = reader.try_fold(
        (HashMap::default(), 0, 0),
        |(mut cache, prev_possible, prev_total_possible), line| -> anyhow::Result<_> {
            cache.clear();
            let combos = possible_combinations(&line?, &towels, &mut cache);

            Ok((
                cache,
                prev_possible + combos.min(1),
                prev_total_possible + combos,
            ))
        },
    )?;

    let elapsed = elapsed.elapsed();

    println!("{} possible combos", possible_count);
    println!("{} design compositions", total_possible_combinations);
    println!("took {}ms", elapsed.as_millis());

    Ok(())
}
