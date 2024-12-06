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

impl Direction {
    fn move_by(self, (row, col): (isize, isize), by: isize) -> (isize, isize) {
        match self {
            Self::Up => (row - by, col),
            Self::Right => (row, col + by),
            Self::Down => (row + by, col),
            Self::Left => (row, col - by),
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
        .map(|row| row.chars().count() as isize)
        .next()
        .context("file is empty")?;
    let rows = map.split('\n').filter(|r| !r.is_empty()).count() as isize;

    let original_pos = map
        .split('\n')
        .enumerate()
        .filter_map(|row| row.1.find('^').map(|col| (row.0 as isize, col as isize)))
        .next()
        .context("guard not found")?;

    let has_barricade_at =
        |(row, col): (isize, isize)| map.as_bytes()[(col + row * (cols + 1)) as usize] == b'#';
    let is_out_of_bounds =
        move |(row, col): (isize, isize)| row < 0 || col < 0 || row >= rows || col >= cols;

    let mut trail = HashSet::default();

    // P1
    {
        let mut guard_pos = original_pos;
        let mut dir = Direction::Up;
        loop {
            let next = dir.move_by(guard_pos, 1);

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
    let max_steps = rows * cols - map.chars().filter(|c| *c == '#').count() as isize - 1;

    let loops = trail
        .into_par_iter()
        .filter(|&candidate| {
            let mut guard_pos = original_pos;
            let mut dir = Direction::Up;
            let mut steps = 0;

            loop {
                let next = dir.move_by(guard_pos, 1);

                if is_out_of_bounds(next) {
                    break false;
                }

                if next == candidate || has_barricade_at(next) {
                    dir = dir.rotate_right()
                } else {
                    guard_pos = next;

                    if steps >= max_steps {
                        break true;
                    }
                }

                steps += 1;
            }
        })
        .count();

    println!("{loops} looping additions, {}ms", now.elapsed().as_millis());

    Ok(())
}
