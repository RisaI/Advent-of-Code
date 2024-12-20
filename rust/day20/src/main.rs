use anyhow::Context;
use aoc_utils::{AStarOptions, FxHashMap as HashMap, FxHashSet as HashSet, IVec2, Map2D};

fn manhattan(a: IVec2, b: IVec2) -> usize {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as usize
}

fn shortest_dist(path: &[IVec2], mut from: usize, mut to: usize) -> (usize, usize) {
    loop {
        let from_v = path[from];
        let to_v = path[to];

        let now = manhattan(from_v, to_v);

        if now > manhattan(from_v, path[(to + 1).clamp(0, path.len() - 1)]) {
            to += 1;
        } else if now > manhattan(from_v, path[to.saturating_sub(1).clamp(0, path.len() - 1)]) {
            to -= 1;
        } else if now > manhattan(path[(from + 1).clamp(0, path.len() - 1)], to_v) {
            from += 1;
        } else if now > manhattan(path[from.saturating_sub(1).clamp(0, path.len() - 1)], to_v) {
            from -= 1;
        } else {
            break;
        }
    }

    (from, to)
}

fn find_shortcuts(path: &[IVec2], max_cheat_steps: usize) -> HashMap<usize, usize> {
    let mut cheats = HashSet::<(usize, usize)>::default();

    for (i, pos) in path.iter().copied().enumerate() {
        for (j, next) in path.iter().copied().enumerate().skip(i + 1) {
            let dist = manhattan(pos, next);

            if dist > max_cheat_steps {
                continue;
            }

            let improvement = (j - i) - dist;

            if improvement > 0 {
                cheats.insert(shortest_dist(path, i, j));
            }
        }
    }

    cheats
        .into_iter()
        .fold(HashMap::default(), |mut result, (i, j)| {
            let improvement = (j - i) - manhattan(path[i], path[j]);

            if improvement > 0 {
                *result.entry(improvement).or_default() += 1;
            }

            result
        })
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

    let shortcut_counts = find_shortcuts(&fastest, 3);

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

    println!("{:?}", shortcut_counts);

    for (by, expected) in [(50, 32)] {
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

    let map = Map2D::read_file("data.txt", |(c, pos)| {
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
        find_shortcuts(&fastest, 3)
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
        find_shortcuts(&fastest, 21)
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
