use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;

fn least_entanglement(packages: &[usize], target: usize) -> Option<usize> {
    if target == 0 {
        return Some(1);
    }

    if packages.iter().copied().sum::<usize>() < target {
        return None;
    }

    packages
        .iter()
        .enumerate()
        .filter(|(_, &v)| v <= target)
        .filter_map(|(i, &v)| {
            least_entanglement(&packages[(i + 1)..], target - v).map(|b| b.saturating_mul(v))
        })
        .min()
}

fn main() -> anyhow::Result<()> {
    let packages = {
        let mut data = BufReader::new(File::open("data.txt")?).lines().try_fold(
            vec![],
            |mut state, line| -> anyhow::Result<_> {
                let line = line?;

                if !line.is_empty() {
                    state.push(line.parse::<usize>()?);
                }

                Ok(state)
            },
        )?;

        data.sort();
        data.reverse();

        data
    };

    let sum = packages.iter().copied().sum::<usize>();

    println!(
        "least entanglement (p1) = {}",
        least_entanglement(&packages, sum / 3).context("solution not found")?
    );

    println!(
        "least entanglement (p2) = {}",
        least_entanglement(&packages, sum / 4).context("solution not found")?
    );

    Ok(())
}
