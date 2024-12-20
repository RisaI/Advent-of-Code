use anyhow::Context;
use aoc_utils::{AStarOptions, FxHashSet, IVec2, Map2D};

fn manhattan(a: IVec2, b: IVec2) -> usize {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as usize
}

fn find_shortcuts(path: &[IVec2], by: usize, max_cheat_steps: usize, _map: &Map2D<bool>) -> usize {
    let mut cheats = FxHashSet::default();

    for (i, pos) in path.iter().copied().enumerate() {
        for steps in 2..=max_cheat_steps {
            let next_idx = i + by + steps;

            if let Some(&next) = path.get(next_idx) {
                if cheats.contains(&next) {
                    break;
                }

                if manhattan(next, pos) == steps {
                    for y in 0.._map.height() {
                        for x in 0.._map.width() {
                            let dp = IVec2::new(x as i32, y as i32);

                            print!(
                                "{}",
                                if next == dp {
                                    '1'
                                } else if pos == dp {
                                    '2'
                                } else if path.contains(&dp) {
                                    'O'
                                } else if matches!(_map.get(dp), Some(true)) {
                                    '#'
                                } else {
                                    '.'
                                }
                            );
                        }
                        println!();
                    }

                    println!();
                    cheats.insert(next);
                    break;
                }
            }
        }
    }

    println!();

    cheats.len()
}

#[test]
fn p1_example_works() {
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

    let fastest = map
        .a_star(AStarOptions::new(start, end))
        .context("no base solution found")
        .unwrap();

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
            find_shortcuts(&fastest, by, 3, &map),
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
        "shurtcuts of 100 steps through 2 walls: {}",
        (100..fastest.len())
            .map(|v| find_shortcuts(&fastest, v, 3, &map))
            .sum::<usize>()
    );

    Ok(())
}
