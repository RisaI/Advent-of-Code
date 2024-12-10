use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{bail, Result};

struct Map {
    width: usize,
    height_map: Box<[u8]>,
}

impl Map {
    pub fn parse_file(path: impl AsRef<Path>) -> Result<Self> {
        let mut height_map = vec![];

        let reader = BufReader::new(File::open(path)?);
        let mut width = 0;

        for line in reader.lines() {
            let line = line?;

            if line.is_empty() {
                continue;
            }

            if width == 0 {
                width = line.len();
            }

            if line.len() != width {
                bail!("inconsistent line width");
            }

            height_map.extend(line.as_bytes().iter().copied().map(|h| h - b'0'));
        }

        debug_assert_eq!(height_map.len() % width, 0);

        Ok(Self {
            width,
            height_map: height_map.into_boxed_slice(),
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height_map.len() / self.width()
    }

    pub fn get_height(&self, x: isize, y: isize) -> Option<u8> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        if x >= self.width() || y >= self.height() {
            return None;
        }

        self.height_map.get(x + y * self.width()).copied()
    }

    pub fn find_trailheads(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.height_map.iter().enumerate().filter_map(|(idx, h)| {
            if *h == 0 {
                Some(((idx % self.width()) as isize, (idx / self.width()) as isize))
            } else {
                None
            }
        })
    }

    pub fn find_trail_score(&self, x: isize, y: isize, unique_peaks: bool) -> usize {
        let mut tops = 0;
        let mut visited = HashSet::<(isize, isize)>::default();

        fn inner(
            x: isize,
            y: isize,
            unique_peaks: bool,
            expected_height: u8,
            map: &Map,
            tops: &mut usize,
            visited: &mut HashSet<(isize, isize)>,
        ) {
            let Some(height) = map.get_height(x, y) else {
                return;
            };

            if height != expected_height {
                return;
            }

            if unique_peaks && !visited.insert((x, y)) {
                return;
            }

            if height >= 9 {
                *tops += 1;
                return;
            }

            inner(x + 1, y, unique_peaks, height + 1, map, tops, visited);
            inner(x, y + 1, unique_peaks, height + 1, map, tops, visited);
            inner(x - 1, y, unique_peaks, height + 1, map, tops, visited);
            inner(x, y - 1, unique_peaks, height + 1, map, tops, visited);
        }

        inner(x, y, unique_peaks, 0, self, &mut tops, &mut visited);

        tops
    }
}

#[test]
fn example_werks() {
    let map = Map {
        width: 8,
        height_map: vec![
            8, 9, 0, 1, 0, 1, 2, 3, 7, 8, 1, 2, 1, 8, 7, 4, 8, 7, 4, 3, 0, 9, 6, 5, 9, 6, 5, 4, 9,
            8, 7, 4, 4, 5, 6, 7, 8, 9, 0, 3, 3, 2, 0, 1, 9, 0, 1, 2, 0, 1, 3, 2, 9, 8, 0, 1, 1, 0,
            4, 5, 6, 7, 3, 2,
        ]
        .into_boxed_slice(),
    };

    let score = map
        .find_trailheads()
        .map(|(x, y)| map.find_trail_score(x, y, true))
        .sum::<usize>();

    assert_eq!(score, 36);
}

#[test]
fn finds_correct_peaks() {
    let data = Map::parse_file("data.txt").unwrap();

    assert_eq!(data.find_trail_score(46, 32, true), 3);
}

fn main() -> Result<()> {
    let map = Map::parse_file("data.txt")?;

    let (p1, p2) = map.find_trailheads().fold((0, 0), |(p1, p2), (x, y)| {
        (
            p1 + map.find_trail_score(x, y, true),
            p2 + map.find_trail_score(x, y, false),
        )
    });

    println!("trailhead score (p1) sum = {p1}");
    println!("trailhead score (p2) sum = {p2}");

    Ok(())
}
