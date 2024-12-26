use aoc_utils::{IVec2 as Vec2, Map2D};
use rustc_hash::FxHashSet as HashSet;

type Map = Map2D<u8>;

fn calculate_fence_price(map: &Map) -> (usize, usize) {
    let mut bitmap = Map2D::<bool>::new(map.width(), map.height());
    let mut known_sides = HashSet::<(Vec2, Vec2)>::default();

    fn find_side_edge(mut pos: Vec2, dir: Vec2, c: u8, map: &Map) -> Vec2 {
        let movement: Vec2 = if dir.y == 0 {
            (0, -1).into()
        } else {
            (-1, 0).into()
        };

        loop {
            match map.get(pos + movement) {
                Some(&v) if v == c => return pos,
                _ => {
                    if map.get(pos + movement + dir) != Some(&c) {
                        return pos;
                    }
                }
            };

            pos += movement;
        }
    }

    fn flood_plot(
        pos: Vec2,
        c: u8,
        map: &Map,
        bitmap: &mut Map2D<bool>,
        known_sides: &mut HashSet<(Vec2, Vec2)>,
    ) -> Option<(usize, usize, usize)> {
        if *map.get(pos)? != c {
            return None;
        }

        if bitmap[pos] {
            return Some((0, 0, 0));
        }

        bitmap.set(pos, true);

        Some(
            [
                Vec2::new(1, 0),
                Vec2::new(0, 1),
                Vec2::new(-1, 0),
                Vec2::new(0, -1),
            ]
            .into_iter()
            .map(
                |dir| match flood_plot(pos + dir, c, map, bitmap, known_sides) {
                    Some(v) => v,
                    None => (
                        0,
                        1,
                        if known_sides.insert((find_side_edge(pos + dir, -dir, c, map), dir)) {
                            1
                        } else {
                            0
                        },
                    ),
                },
            )
            .fold((1, 0, 0), |(area, perimeter, sides), (a, b, c)| {
                (area + a, perimeter + b, sides + c)
            }),
        )
    }

    (0..map.height())
        .flat_map(|y| (0..map.width()).map(move |x| (x, y)))
        .filter_map(|(x, y)| {
            let pos = Vec2::new(x as i32, y as i32);

            if bitmap[pos] {
                return None;
            }

            known_sides.clear();

            flood_plot(
                pos,
                *map.get(pos).unwrap(),
                map,
                &mut bitmap,
                &mut known_sides,
            )
        })
        .fold(
            (0, 0),
            |(nondiscounted, discounted), (area, perimeter, sides)| {
                (nondiscounted + perimeter * area, discounted + sides * area)
            },
        )
}

#[test]
fn example_just_werks() {
    let map = Map::read_str(
        "AAAA
BBCD
BBCC
EEEC",
        |c: char| c as u8 - b'A',
    )
    .unwrap();

    assert_eq!(calculate_fence_price(&map), (140, 80));
}

fn main() -> anyhow::Result<()> {
    let map = Map::read_file("input.txt", |c: char| c as u8 - b'A')?;

    let start = std::time::Instant::now();
    let (nondiscounted, discounted) = calculate_fence_price(&map);
    let elapsed = start.elapsed();

    println!("total fence price is {}", nondiscounted);
    println!("discounted fence price is {}", discounted);
    println!("took {}us", elapsed.as_micros());

    Ok(())
}
