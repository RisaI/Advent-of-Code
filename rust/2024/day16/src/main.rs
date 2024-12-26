use std::collections::{HashMap, HashSet};

use anyhow::bail;
use aoc_utils::{IVec2, Map2D};

fn find_least_score(pos: IVec2, dir: IVec2, map: &Map2D<bool>, end: IVec2) -> Option<usize> {
    fn inner(
        pos: IVec2,
        dir: IVec2,
        score: usize,
        branched: bool,
        map: &Map2D<bool>,
        end: IVec2,
        memory: &mut HashMap<IVec2, usize>,
    ) -> Option<usize> {
        if branched {
            if let Some(&prev) = memory.get(&pos) {
                if prev <= score {
                    return None;
                }
            }

            memory.insert(pos, score);
        }

        let mut branches = vec![];
        let normal = IVec2::new(dir.y, -dir.x);

        for i in 0.. {
            let pos = pos + i * dir;

            if let Some(true) = map.get(pos) {
                break;
            }

            if pos == end {
                return Some(score + i as usize);
            }

            for normal in [normal, -normal] {
                if let Some(false) = map.get(pos + normal) {
                    branches.push((pos + normal, normal, score + i as usize + 1_001));
                }
            }
        }

        branches
            .into_iter()
            .rev()
            .filter_map(|(pos, dir, score)| inner(pos, dir, score, true, map, end, memory))
            .min()
    }

    inner(pos, dir, 0, false, map, end, &mut HashMap::default())
}

fn find_tile_count(start: IVec2, dir: IVec2, map: &Map2D<bool>, end: IVec2, cap: usize) -> usize {
    fn inner(
        pos: IVec2,
        dir: IVec2,
        map: &Map2D<bool>,
        end: IVec2,
        cap: i32,
        tiles: &mut HashSet<IVec2>,
        known_caps: &mut HashMap<IVec2, i32>,
    ) -> bool {
        if cap < 0 {
            return false;
        }

        if known_caps.get(&pos).copied().unwrap_or_default() > cap {
            return false;
        }

        known_caps.insert(pos, cap);

        let normal = IVec2::new(dir.y, -dir.x);
        let mut matched = false;

        for i in 0..=cap {
            let next_pos = pos + i * dir;

            if map.get(next_pos).copied() != Some(false) {
                break;
            }

            if next_pos == end {
                tiles.extend((0..=i).map(|i| pos + i * dir));
                return true;
            }

            for normal in [normal, -normal] {
                if let Some(false) = map.get(next_pos + normal) {
                    if inner(
                        next_pos + normal,
                        normal,
                        map,
                        end,
                        cap - 1001 - i,
                        tiles,
                        known_caps,
                    ) {
                        matched = true;
                        tiles.extend((0..=i).map(|i| pos + i * dir));
                    }
                }
            }
        }

        matched
    }

    let mut tiles = HashSet::from([start]);

    inner(
        start,
        dir,
        map,
        end,
        cap as i32,
        &mut tiles,
        &mut HashMap::default(),
    );

    tiles.len()
}

fn main() -> anyhow::Result<()> {
    let mut start_pos = IVec2::default();
    let mut end_pos = IVec2::default();

    let map = Map2D::read_file("input.txt", |(c, pos): (char, IVec2)| {
        match c {
            'S' => start_pos = pos,
            'E' => end_pos = pos,
            _ => (),
        };

        c == '#'
    })?;

    let Some(score) = find_least_score(start_pos, IVec2::new(1, 0), &map, end_pos) else {
        bail!("no solution found");
    };

    println!("least score = {score}");
    println!(
        "unique tiles = {}",
        find_tile_count(start_pos, IVec2::new(1, 0), &map, end_pos, score)
    );

    Ok(())
}
