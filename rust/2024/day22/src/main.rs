use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use rustc_hash::{FxHashMap, FxHashSet};

fn rotate_secret(secret: usize) -> usize {
    fn prune(a: usize) -> usize {
        a % 16777216
    }

    let secret = prune(secret ^ (secret << 6));
    let secret = prune(secret ^ (secret >> 5));

    prune(secret ^ (secret << 11))
}

#[test]
fn p1_rotate_example() {
    let mut a = 123;

    for expected in [
        15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    ] {
        a = rotate_secret(a);

        assert_eq!(a, expected);
    }
}

fn seq_tables(seeds: &[usize]) -> FxHashMap<[i8; 4], usize> {
    let mut result = FxHashMap::default();

    seeds.iter().for_each(|&s| {
        let mut encountered = FxHashSet::default();
        let mut diffs = VecDeque::new();
        let mut prev = (s % 10) as i8;

        (0..2000)
            .scan(s, |state, _| {
                *state = rotate_secret(*state);
                Some((*state % 10) as i8)
            })
            .for_each(|next| {
                while diffs.len() >= 4 {
                    diffs.pop_front();
                }
                diffs.push_back(next - prev);

                prev = next;

                if diffs.len() == 4 {
                    let seq = std::array::from_fn::<i8, 4, _>(|i| diffs[i]);

                    if encountered.insert(seq) {
                        *result.entry(seq).or_default() += next as usize;
                    }
                }
            });
    });

    result
}
#[test]
fn p2_example() {
    let seeds = vec![1, 2, 3, 2024];
    let sequence = [-2, 1, -1, 3];

    assert_eq!(seq_tables(&seeds).get(&sequence).copied(), Some(23));
}

fn main() -> anyhow::Result<()> {
    let seeds = BufReader::new(File::open("input.txt")?).lines().try_fold(
        vec![],
        |mut state, line| -> anyhow::Result<_> {
            let line = line?;

            if !line.is_empty() {
                state.push(line.parse::<usize>()?);
            }

            Ok(state)
        },
    )?;

    let sum = seeds
        .iter()
        .map(|&s| (0..2000).fold(s, |s, _| rotate_secret(s)))
        .sum::<usize>();

    println!("sum of seeds after 2000 runs = {sum}");

    let max_bananas = seq_tables(&seeds).values().max().copied().unwrap();

    println!("max bananas you can get = {max_bananas}");

    Ok(())
}
