use anyhow::Context;
use fxhash::FxHashSet as HashSet;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

type Coord = usize;
type Vec2 = (Coord, Coord);

impl Direction {
    fn move_pos(self, (row, col): Vec2) -> Option<Vec2> {
        match self {
            Self::Up => row.checked_sub(1).map(|row| (row, col)),
            Self::Right => col.checked_add(1).map(|col| (row, col)),
            Self::Down => row.checked_add(1).map(|row| (row, col)),
            Self::Left => col.checked_sub(1).map(|col| (row, col)),
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let map = std::fs::read_to_string("data.txt")?;

    anyhow::ensure!(map.is_ascii(), "the map should be ASCII");

    let cols = map
        .split('\n')
        .map(|row| row.chars().count() as Coord)
        .next()
        .context("file is empty")?;
    let rows = map.split('\n').filter(|r| !r.is_empty()).count() as Coord;

    let original_pos = map
        .split('\n')
        .enumerate()
        .filter_map(|row| row.1.find('^').map(|col| (row.0 as Coord, col as Coord)))
        .next()
        .context("guard not found")?;

    let has_barricade_at =
        |(row, col): Vec2| map.as_bytes()[(col + row * (cols + 1)) as usize] == b'#';
    let is_out_of_bounds = move |(row, col): Vec2| row >= rows || col >= cols;

    let mut trail = HashSet::default();

    // P1
    {
        let mut guard_pos = original_pos;
        let mut dir = Direction::Up;
        loop {
            let Some(next) = dir.move_pos(guard_pos) else {
                break;
            };

            if is_out_of_bounds(next) {
                break;
            }

            if has_barricade_at(next) {
                dir = dir.rotate_right()
            } else {
                guard_pos = next;
                trail.insert(guard_pos);
            }
        }

        println!("visited {} fields", trail.len() + 1);
    }

    // P2
    let now = std::time::Instant::now();

    let loops = trail
        .into_par_iter()
        .filter(|&candidate| {
            let mut guard_pos = original_pos;
            let mut dir = Direction::Up;
            let mut turning_points = HashSet::default();
            let mut rotated = false;

            loop {
                let Some(next) = dir.move_pos(guard_pos) else {
                    break false;
                };

                if is_out_of_bounds(next) {
                    break false;
                }

                if next == candidate || has_barricade_at(next) {
                    dir = dir.rotate_right();
                    rotated = true;
                } else {
                    if rotated {
                        rotated = false;

                        if !turning_points.insert(guard_pos) {
                            break true;
                        }
                    }

                    guard_pos = next;
                }
            }
        })
        .count();

    println!("{loops} looping additions, {}ms", now.elapsed().as_millis());

    Ok(())
}
