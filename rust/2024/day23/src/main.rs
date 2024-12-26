use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{bail, Context};
use rustc_hash::{FxHashMap, FxHashSet as HashSet};

fn main() -> anyhow::Result<()> {
    let map = BufReader::new(File::open("input.txt")?).lines().try_fold(
        FxHashMap::<String, HashSet<String>>::default(),
        |mut state, line| -> anyhow::Result<_> {
            let line = line?;

            if !line.is_empty() {
                let mut line = line.split('-').map(String::from);

                let [Some(a), Some(b), None] =
                    std::array::from_fn(|_| line.next().map(String::from))
                else {
                    bail!("wrong format");
                };

                state.entry(a.clone()).or_default().insert(b.clone());
                state.entry(b).or_default().insert(a);
            }

            Ok(state)
        },
    )?;

    let start = std::time::Instant::now();

    // P1
    {
        let mut known = HashSet::default();

        for (a, values) in map.iter().filter(|(k, _)| k.starts_with('t')) {
            for b in values.iter() {
                for c in &map[b] {
                    if values.contains(c) {
                        let mut triplet = [a.as_str(), b.as_str(), c.as_str()];

                        triplet.sort();

                        known.insert(triplet);
                    }
                }
            }
        }

        println!("{} triplets with a puter starting with 't'", known.len());
    }

    // P2
    {
        let mut groups: Vec<HashSet<String>> = vec![];

        for (a, v) in map.iter() {
            for group in groups.iter_mut() {
                if group.intersection(v).count() == group.len() {
                    group.insert(a.clone());
                }
            }

            for b in v {
                if !groups.iter().any(|g| g.contains(a) && g.contains(b)) {
                    groups.push(HashSet::from_iter([a.clone(), b.clone()]));
                }
            }
        }

        let mut max_group = groups
            .iter()
            .max_by_key(|g| g.len())
            .context("no groups found")?
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        max_group.sort();

        println!("{}", max_group.join(","));
    }

    let elapsed = start.elapsed();
    println!("took {}ms", elapsed.as_millis());

    Ok(())
}
