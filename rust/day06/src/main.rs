use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::bail;
use fxhash::FxHashSet;
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
    let reader = BufReader::new(File::open("data.txt")?);
    let mut cols = 0isize;
    let mut rows = 0isize;

    let mut original_pos = Option::<(isize, isize)>::None;
    let mut barricades = FxHashSet::default();

    for (row_idx, line) in reader.lines().enumerate() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let char_count = line.chars().count() as isize;

        if cols == 0 {
            cols = char_count;
        }

        if cols != char_count {
            bail!("inconsistent line width");
        }

        rows = row_idx as isize + 1;

        for (col_idx, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    barricades.insert((row_idx as isize, col_idx as isize));
                }
                '^' => {
                    original_pos = Some((row_idx as isize, col_idx as isize));
                }
                _ => {
                    // noop
                }
            }
        }
    }

    let Some(original_pos) = original_pos else {
        bail!("guard not found");
    };

    let is_out_of_bounds =
        move |(row, col): (isize, isize)| row < 0 || col < 0 || row >= rows || col >= cols;

    let mut trail = FxHashSet::default();
    trail.insert(original_pos);

    // P1
    {
        let mut guard_pos = original_pos;
        let mut dir = Direction::Up;
        loop {
            let next = dir.move_by(guard_pos, 1);

            if is_out_of_bounds(next) {
                break;
            }

            if barricades.contains(&next) {
                dir = dir.rotate_right()
            } else {
                guard_pos = next;
                trail.insert(guard_pos);
            }
        }

        println!("visited {} fields", trail.len());
    }

    // P2
    let loops = trail
        .into_par_iter()
        .filter(|&candidate| {
            if candidate == original_pos {
                return false;
            }

            let mut trail = HashSet::<((isize, isize), Direction)>::default();
            let mut guard_pos = original_pos;
            let mut dir = Direction::Up;

            loop {
                let next = dir.move_by(guard_pos, 1);

                if is_out_of_bounds(next) {
                    break false;
                }

                if next == candidate || barricades.contains(&next) {
                    dir = dir.rotate_right()
                } else {
                    guard_pos = next;

                    if !trail.insert((guard_pos, dir)) {
                        break true;
                    }
                }
            }
        })
        .count();

    println!("{loops} looping additions");

    Ok(())
}
