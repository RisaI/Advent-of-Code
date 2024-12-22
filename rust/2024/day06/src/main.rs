use anyhow::Context;
use rayon::prelude::*;
use rustc_hash::FxHashSet as HashSet;

type Coord = i32;
type Vec2 = glam::IVec2;

fn main() -> anyhow::Result<()> {
    let map = std::fs::read_to_string("data.txt")?;

    anyhow::ensure!(map.is_ascii(), "the map should be ASCII");

    let width = map
        .split('\n')
        .map(|row| row.chars().count() as Coord)
        .next()
        .context("file is empty")?;
    let height = map.split('\n').filter(|r| !r.is_empty()).count() as Coord;

    let original_pos = map
        .split('\n')
        .enumerate()
        .filter_map(|row| {
            row.1
                .find('^')
                .map(|col| Vec2::new(col as Coord, row.0 as Coord))
        })
        .next()
        .context("guard not found")?;

    let has_barricade_at = |v: Vec2| map.as_bytes()[(v.x + v.y * (width + 1)) as usize] == b'#';
    let is_out_of_bounds = move |v: Vec2| v.x < 0 || v.y < 0 || v.y >= height || v.x >= width;

    let mut trail = HashSet::default();

    // P1
    {
        let mut guard_pos = original_pos;
        let mut dir = Vec2::new(0, -1);

        loop {
            let next = guard_pos + dir;

            if is_out_of_bounds(next) {
                break;
            }

            if has_barricade_at(next) {
                dir = Vec2::new(-dir.y, dir.x);
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
            let mut dir = Vec2::new(0, -1);
            let mut turning_points = HashSet::default();
            let mut rotated = false;

            loop {
                let next = guard_pos + dir;

                if is_out_of_bounds(next) {
                    break false;
                }

                if next == candidate || has_barricade_at(next) {
                    dir = Vec2::new(-dir.y, dir.x);
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
