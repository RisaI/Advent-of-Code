use anyhow::Context;
use aoc_utils::{AStarOptions, FxHashMap as HashMap, IVec2, Map2D};

fn manhattan(a: IVec2, b: IVec2) -> usize {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as usize
}

fn improvement(path: &[IVec2], i: usize, j: usize) -> usize {
    i.abs_diff(j) - manhattan(path[i], path[j])
}

fn find_shortcuts(path: &[IVec2], max_cheat_steps: usize) -> HashMap<usize, usize> {
    let mut cheats = HashMap::<usize, usize>::default();

    for (i, pos) in path.iter().copied().enumerate() {
        for (j, next) in path.iter().copied().enumerate().skip(i + 1) {
            let dist = manhattan(pos, next);

            if dist > max_cheat_steps {
                continue;
            }

            let improvement = improvement(path, i, j);

            if improvement > 0 {
                *cheats.entry(improvement).or_default() += 1;
            }
        }
    }

    cheats
}

#[cfg(test)]
fn load_example() -> (IVec2, IVec2, Map2D<bool>) {
    let mut start = IVec2::ZERO;
    let mut end = IVec2::ZERO;

    let map = Map2D::read_str(
        "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
        |(c, pos)| {
            match c {
                '#' => return true,
                'S' => {
                    start = pos;
                }
                'E' => {
                    end = pos;
                }
                _ => (),
            };

            false
        },
    )
    .unwrap();

    (start, end, map)
}

#[test]
fn p1_example_works() {
    let (start, end, map) = load_example();

    let fastest = map
        .a_star(AStarOptions::new(start, end))
        .context("no base solution found")
        .unwrap();

    let shortcut_counts = find_shortcuts(&fastest, 2);

    for (by, expected) in [
        (2, 14),
        (4, 14),
        (6, 2),
        (8, 4),
        (10, 2),
        (12, 3),
        (20, 1),
        (36, 1),
        (38, 1),
        (40, 1),
        (64, 1),
    ] {
        assert_eq!(
            shortcut_counts.get(&by).copied().unwrap_or_default(),
            expected,
            "there should be {} shortcuts for {} picoseconds",
            expected,
            by
        )
    }
}

#[test]
fn p2_example_works() {
    let (start, end, map) = load_example();

    let fastest = map
        .a_star(AStarOptions::new(start, end))
        .context("no base solution found")
        .unwrap();

    let shortcut_counts = find_shortcuts(&fastest, 21);

    for (by, expected) in [
        (50, 32),
        (52, 31),
        (54, 29),
        (56, 39),
        (58, 25),
        (60, 23),
        (62, 20),
        (64, 19),
        (66, 12),
        (68, 14),
        (70, 12),
        (72, 22),
        (74, 4),
        (76, 3),
    ] {
        assert_eq!(
            shortcut_counts.get(&by).copied().unwrap_or_default(),
            expected,
            "there should be {} shortcuts for {} picoseconds",
            expected,
            by
        )
    }
}

fn main() -> anyhow::Result<()> {
    let mut start = IVec2::ZERO;
    let mut end = IVec2::ZERO;

    let map = Map2D::read_file("input.txt", |(c, pos)| {
        match c {
            '#' => return true,
            'S' => {
                start = pos;
            }
            'E' => {
                end = pos;
            }
            _ => (),
        };

        false
    })?;

    let fastest = map
        .a_star(AStarOptions::new(start, end))
        .context("no base solution found")?;

    println!("fastest path: {}", fastest.len() - 1);
    println!(
        "shortcuts of 100 steps through 2 walls: {}",
        find_shortcuts(&fastest, 2)
            .into_iter()
            .filter_map(|(improvement, count)| if improvement >= 100 {
                Some(count)
            } else {
                None
            })
            .sum::<usize>()
    );

    println!(
        "shortcuts of 100 steps through 20 walls: {}",
        find_shortcuts(&fastest, 20)
            .into_iter()
            .filter_map(|(improvement, count)| if improvement >= 100 {
                Some(count)
            } else {
                None
            })
            .sum::<usize>()
    );

    Ok(())
}
