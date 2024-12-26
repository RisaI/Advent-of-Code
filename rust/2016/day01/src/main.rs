use std::collections::HashSet;

use anyhow::{bail, Context};
use aoc_utils::{manhattan, IVec2};

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("input.txt")?;

    let (corrected, _, pos, _) = data
        .split(", ")
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .try_fold(
            (None, HashSet::new(), IVec2::ZERO, IVec2::new(0, -1)),
            |(mut twice, mut visited, pos, dir), instruction| {
                let ch = instruction.chars().next().unwrap();
                let dist = instruction[1..].parse::<i32>()?;

                let dir = match ch {
                    'L' => IVec2::new(-dir.y, dir.x),
                    'R' => IVec2::new(dir.y, -dir.x),
                    _ => bail!("unknown instruction '{}'", ch),
                };

                for pos in (1..=dist).map(|i| pos + dir * i) {
                    if !visited.insert(pos) {
                        twice = twice.or(Some(pos));
                    }
                }

                Ok((twice, visited, pos + dir * dist, dir))
            },
        )?;

    println!("the distance is {} blocks", manhattan(pos, IVec2::ZERO));
    println!(
        "the corrected distance is {} blocks",
        manhattan(corrected.context("no solution for p2")?, IVec2::ZERO)
    );

    Ok(())
}
