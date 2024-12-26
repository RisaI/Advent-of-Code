use aoc_utils::{IVec2, Map2D};

fn animate(map: &mut Map2D<bool>, buffer: &mut Map2D<bool>) {
    for y in 0..(map.height() as i32) {
        for x in 0..(map.width() as i32) {
            let pos = IVec2::new(x, y);

            let neighbours = ((-1)..=1)
                .flat_map(|dy| ((-1)..=1).map(move |dx| IVec2::new(dx, dy)))
                .filter(|v| (v.x != 0 || v.y != 0) && matches!(map.get(*v + pos), Some(true)))
                .count();

            if let Some(true) = map.get(pos) {
                buffer.set(pos, (2..=3).contains(&neighbours));
            } else {
                buffer.set(pos, neighbours == 3);
            }
        }
    }

    std::mem::swap(map, buffer);
}

fn main() -> anyhow::Result<()> {
    let mut map = Map2D::read_file("input.txt", |l: char| l == '#')?;
    let mut buffer = map.clone();

    // P1
    {
        let mut map = map.clone();

        for _ in 0..100 {
            animate(&mut map, &mut buffer);
        }

        println!("{} light on after 100 steps", map.find(|_, v| *v).count());
    }

    // P2
    {
        fn light_corners(map: &mut Map2D<bool>) {
            let w = map.width();
            let h = map.height();

            [0, w - 1]
                .into_iter()
                .flat_map(|x| {
                    [0, h - 1]
                        .into_iter()
                        .map(move |y| IVec2::new(x as i32, y as i32))
                })
                .for_each(|p| map.set(p, true));
        }

        light_corners(&mut map);

        for _ in 0..100 {
            animate(&mut map, &mut buffer);
            light_corners(&mut map);
        }

        println!(
            "{} light on after 100 steps with corners on",
            map.find(|_, v| *v).count()
        );
    }

    Ok(())
}
