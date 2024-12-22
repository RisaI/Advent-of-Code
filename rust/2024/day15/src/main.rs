use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    iter::once,
};

use aoc_utils::{IVec2, Map2D};

struct Crate {
    pos: IVec2,
    width: i32,
}

impl Crate {
    pub fn contains(&self, point: IVec2) -> bool {
        let point = point - self.pos;

        point.y == 0 && point.x >= 0 && point.x < self.width
    }

    pub fn generate_push_positions(&self, dir: IVec2) -> Vec<IVec2> {
        match (dir.x, dir.y) {
            (1, 0) => vec![self.pos + IVec2::new(self.width, 0)],
            (-1, 0) => vec![self.pos + IVec2::new(-1, 0)],
            (0, 1) => (0..self.width)
                .map(|d| self.pos + IVec2::new(d, 1))
                .collect(),
            (0, -1) => (0..self.width)
                .map(|d| self.pos + IVec2::new(d, -1))
                .collect(),
            _ => panic!("not a direction"),
        }
    }
}

fn push_crates(pos: IVec2, dir: IVec2, crates: &mut [Crate], map: &Map2D<bool>) -> bool {
    let mut affected = HashSet::new();
    let mut queue = VecDeque::from([pos]);

    while let Some(pos) = queue.pop_front() {
        if let Some(true) | None = map.get(pos) {
            return false;
        }

        if let Some(idx) = crates.iter().position(|c| c.contains(pos)) {
            if affected.insert(idx) {
                queue.extend(crates[idx].generate_push_positions(dir));
            }
        }
    }

    for idx in affected {
        crates[idx].pos += dir;
    }

    true
}

#[allow(dead_code)]
fn print_map(map: &Map2D<bool>, crates: &[Crate], player: IVec2) {
    let mut s = (0..map.height())
        .flat_map(|y| {
            (0..map.width())
                .map(move |x| {
                    if let Some(true) = map.get(IVec2::new(x as i32, y as i32)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .chain(once('\n'))
        })
        .collect::<String>();

    for c in crates {
        let start = c.pos.x as usize + c.pos.y as usize * (map.width() + 1);

        s.replace_range(
            start..(start + c.width as usize),
            match c.width {
                2 => "[]",
                _ => "O",
            },
        );
    }

    let start = player.x as usize + player.y as usize * (map.width() + 1);
    s.replace_range(start..=start, "@");

    println!("{s}");
}

fn main() -> anyhow::Result<()> {
    let map = Map2D::read_file("data.txt", |c: char| c == '#')?;
    let mut crates = vec![];
    let mut player = IVec2::new(0, 0);

    let mut reader = BufReader::new(File::open("data.txt")?).lines();

    for (y, line) in reader.by_ref().enumerate() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        for (x, ch) in line.chars().enumerate() {
            match ch {
                '@' => {
                    player = IVec2::new(x as i32, y as i32);
                }
                'O' => {
                    crates.push(Crate {
                        pos: IVec2::new(x as i32, y as i32),
                        width: 1,
                    });
                }
                _ => (),
            }
        }
    }

    let instructions =
        reader.try_fold(String::new(), |mut prev, line| -> anyhow::Result<String> {
            let line = line?;

            prev.push_str(&line);

            Ok(prev)
        })?;

    for width_factor in [1, 2] {
        let mut player = player * IVec2::new(width_factor, 1);
        let mut crates = crates
            .iter()
            .map(|b| Crate {
                pos: b.pos * IVec2::new(width_factor, 1),
                width: width_factor,
            })
            .collect::<Vec<_>>();
        let map = {
            let mut m = Map2D::new(map.width() * width_factor as usize, map.height());

            for y in 0..map.height() {
                for x in 0..map.width() {
                    if let Some(true) = map.get(IVec2::new(x as i32, y as i32)) {
                        for i in 0..width_factor {
                            m.set(IVec2::new(width_factor * x as i32 + i, y as i32), true);
                        }
                    }
                }
            }

            m
        };

        for ch in instructions.chars() {
            let d = match ch {
                '>' => IVec2::new(1, 0),
                'v' => IVec2::new(0, 1),
                '<' => IVec2::new(-1, 0),
                '^' => IVec2::new(0, -1),
                _ => continue,
            };

            if push_crates(player + d, d, &mut crates, &map) {
                player += d
            }
        }

        let sum = crates.iter().map(|b| b.pos.y * 100 + b.pos.x).sum::<i32>();

        // print_map(&map, &crates, player);
        println!("the GPS sum is {sum} for width factor {width_factor}");
    }

    Ok(())
}
