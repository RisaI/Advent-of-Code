use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::bail;
use clap::Parser;

#[derive(Parser)]
struct Options {
    #[arg(long)]
    pub disable_dampener: bool,
}

fn main() -> anyhow::Result<()> {
    let opts = Options::parse();
    let reader = BufReader::new(File::open("data.txt")?);

    let mut valid = 0;

    for (idx, line) in reader.lines().enumerate() {
        let line = line?;

        let mut split = line.split(' ').map(|v| v.parse::<isize>());

        let Some(Ok(mut prev)) = split.next() else {
            bail!("invalid report at line {}", idx + 1);
        };

        let mut dir = 0;
        let mut dampener = !opts.disable_dampener;

        let result: anyhow::Result<()> = split.try_for_each(|entry| {
            let entry = entry?;
            let dist = entry - prev;
            prev = entry;

            if dir == 0 {
                dir = dist.signum();
            }

            let comparison = (|| {
                if dist.signum() != dir {
                    bail!("monotonicity is broken");
                }

                if dist.abs() > 3 {
                    bail!("difference between levels is higher than 3");
                } else if dist.abs() == 0 {
                    bail!("difference between levels is zero");
                }

                Ok(())
            })();

            if comparison.is_err() {
                if !dampener {
                    return comparison;
                }

                dampener = false
            }
            Ok(())
        });

        if result.is_ok() {
            valid += 1;
        }
    }

    println!("{valid} valid reports");

    Ok(())
}
