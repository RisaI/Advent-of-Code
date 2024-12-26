use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use winnow::{
    ascii::{alpha1, digit1},
    error::InputError,
    Parser,
};

struct Reindeer {
    pub name: String,
    pub speed: usize,
    pub run_time: usize,
    pub rest_time: usize,
}

impl Reindeer {
    pub fn distance_after(&self, secs: usize) -> usize {
        let cycle = self.run_time + self.rest_time;
        let full_cycles = secs / cycle;
        let partial_cycle = secs % cycle;

        (full_cycles * self.run_time + partial_cycle.min(self.run_time)) * self.speed
    }
}

impl FromStr for Reindeer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, _, speed, _, run_time, _, rest_time, _) = (
            alpha1::<&str, InputError<&str>>,
            " can fly ",
            digit1.parse_to(),
            " km/s for ",
            digit1.parse_to(),
            " seconds, but then must rest for ",
            digit1.parse_to(),
            " seconds.",
        )
            .parse(s)
            .map_err(|e| anyhow::format_err!("{e}"))?;

        Ok(Self {
            name: name.to_string(),
            speed,
            run_time,
            rest_time,
        })
    }
}

const TARGET_TIME: usize = 2503;

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    let mut reindeers = vec![];

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        reindeers.push(line.parse::<Reindeer>()?);
    }

    let winner = reindeers
        .iter()
        .map(|r| (r, r.distance_after(TARGET_TIME)))
        .max_by_key(|(_, v)| *v)
        .unwrap();

    println!(
        "after {TARGET_TIME} seconds, {} wins at {} km",
        winner.0.name, winner.1
    );

    let scores = (1..=TARGET_TIME)
        .map(|secs| reindeers.iter().map(move |r| r.distance_after(secs)))
        .fold(vec![0; reindeers.len()], |mut scores, distances| {
            distances
                .enumerate()
                .fold((0, vec![]), |(mut prev_max, mut indices), (idx, dist)| {
                    match dist.cmp(&prev_max) {
                        std::cmp::Ordering::Less => (),
                        std::cmp::Ordering::Equal => indices.push(idx),
                        std::cmp::Ordering::Greater => {
                            prev_max = dist;
                            indices.clear();
                            indices.push(idx);
                        }
                    }

                    (prev_max, indices)
                })
                .1
                .into_iter()
                .for_each(|idx| scores[idx] += 1);

            scores
        });

    println!(
        "the winning reindeer has a score of {}",
        scores.into_iter().max().unwrap()
    );

    Ok(())
}
