use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::bail;
use aoc_utils::{AStarOptions, IVec2, Map2D};

#[test]
fn p1_example_works() {
    let map = Map2D::read_str(
        "...#...
..#..#.
....#..
...#..#
..#..#.
.#..#..
#.#....",
        |c: char| c == '#',
    )
    .unwrap();

    assert_eq!(
        map.a_star(AStarOptions::new(
            IVec2::new(0, 0),
            IVec2::new(map.width() as i32 - 1, map.height() as i32 - 1),
        ))
        .map(|p| p.len() - 1),
        Some(22),
    )
}

fn main() -> anyhow::Result<()> {
    let data = BufReader::new(File::open("input.txt")?).lines().try_fold(
        vec![],
        |mut state, line| -> anyhow::Result<_> {
            let line = line?;

            let coords = line
                .split(',')
                .map(|v| v.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()?;

            if coords.len() != 2 {
                bail!("expected two coordinate components");
            }

            state.push(IVec2::new(coords[0], coords[1]));

            Ok(state)
        },
    )?;

    let map = data
        .iter()
        .take(1024)
        .fold(Map2D::new(71, 71), |mut state, &p| {
            state.set(p, true);

            state
        });

    println!(
        "shortest path with 1024 bytes = {}",
        map.a_star(AStarOptions::new(
            IVec2::new(0, 0),
            IVec2::new(map.width() as i32 - 1, map.height() as i32 - 1),
        ))
        .unwrap()
        .len()
            - 1
    );

    let search = data.iter().enumerate().map(|(i, _)| i).collect::<Vec<_>>();

    let p2 = search.binary_search_by(|&idx| {
        let map = data
            .iter()
            .take(idx + 1)
            .fold(Map2D::new(71, 71), |mut state, &p| {
                state.set(p, true);

                state
            });

        match map.a_star(AStarOptions::new(
            IVec2::new(0, 0),
            IVec2::new(map.width() as i32 - 1, map.height() as i32 - 1),
        )) {
            Some(_) => std::cmp::Ordering::Less,
            None => std::cmp::Ordering::Greater,
        }
    });

    println!("path cut off by byte at {}", data[p2.unwrap_err()]);

    Ok(())
}
