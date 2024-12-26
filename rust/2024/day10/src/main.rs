use std::collections::HashSet;

use anyhow::Result;
use aoc_utils::{IVec2 as Vec2, Map2D};

type Map = Map2D<u8>;

pub fn find_trailheads(_: Vec2, v: &u8) -> bool {
    *v == 0
}

pub fn find_trail_score(map: &Map, pos: Vec2, unique_peaks: bool) -> usize {
    let mut tops = 0;
    let mut visited = HashSet::<Vec2>::default();

    fn inner(
        pos: Vec2,
        unique_peaks: bool,
        expected_height: u8,
        map: &Map,
        tops: &mut usize,
        visited: &mut HashSet<Vec2>,
    ) {
        let Some(&height) = map.get(pos) else {
            return;
        };

        if height != expected_height {
            return;
        }

        if unique_peaks && !visited.insert(pos) {
            return;
        }

        if height >= 9 {
            *tops += 1;
            return;
        }

        [
            Vec2::new(1, 0),
            Vec2::new(0, 1),
            Vec2::new(-1, 0),
            Vec2::new(0, -1),
        ]
        .into_iter()
        .for_each(|d| inner(pos + d, unique_peaks, height + 1, map, tops, visited));
    }

    inner(pos, unique_peaks, 0, map, &mut tops, &mut visited);

    tops
}

fn parse_char(c: char) -> u8 {
    c as u8 - b'0'
}

#[test]
fn example_werks() {
    let map = Map::read_str(
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        parse_char,
    )
    .unwrap();

    let score = map
        .find(find_trailheads)
        .map(|(p, _)| find_trail_score(&map, p, true))
        .sum::<usize>();

    assert_eq!(score, 36);
}

#[test]
fn finds_correct_peaks() {
    let data = Map::read_file("input.txt", parse_char).unwrap();

    assert_eq!(find_trail_score(&data, Vec2::new(46, 32), true), 3);
}

fn main() -> Result<()> {
    let map = Map::read_file("input.txt", parse_char)?;

    let (p1, p2) = map.find(find_trailheads).fold((0, 0), |(p1, p2), (p, _)| {
        (
            p1 + find_trail_score(&map, p, true),
            p2 + find_trail_score(&map, p, false),
        )
    });

    println!("trailhead score (p1) sum = {p1}");
    println!("trailhead score (p2) sum = {p2}");

    Ok(())
}
