use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;
use clap::Parser;

#[derive(Parser)]
struct Options {
    #[arg(long)]
    pub disable_dampener: bool,
}

fn main() -> anyhow::Result<()> {
    let opts = Options::parse();
    let reader = BufReader::new(File::open("input.txt")?);

    let mut valid = 0;

    for (idx, line) in reader.lines().enumerate() {
        let levels = line?
            .split(' ')
            .map(|v| v.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()
            .with_context(|| format!("invalid report at line {}", idx + 1))?;

        let mut dir = 0;
        let mut dampener = !opts.disable_dampener;

        if levels
            .windows(2)
            .map(|vals| {
                let dist = vals[1] - vals[0];

                if dir == 0 {
                    dir = dist.signum();
                }

                let valid = dist.signum() == dir && dist.abs() <= 3 && dist.abs() > 0;

                valid || std::mem::replace(&mut dampener, false)
            })
            .all(|v| v)
        {
            valid += 1;
        }
    }

    println!("{valid} valid reports");

    Ok(())
}
