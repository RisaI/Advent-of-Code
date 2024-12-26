use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::LazyLock,
};

use anyhow::bail;
use aoc_utils::IVec2;

static P2_KEYPAD: LazyLock<HashMap<IVec2, char>> = LazyLock::new(|| {
    HashMap::from_iter([
        (IVec2::new(2, 0), '1'),
        (IVec2::new(1, 1), '2'),
        (IVec2::new(2, 1), '3'),
        (IVec2::new(3, 1), '4'),
        (IVec2::new(0, 2), '5'),
        (IVec2::new(1, 2), '6'),
        (IVec2::new(2, 2), '7'),
        (IVec2::new(3, 2), '8'),
        (IVec2::new(4, 2), '9'),
        (IVec2::new(1, 3), 'A'),
        (IVec2::new(2, 3), 'B'),
        (IVec2::new(3, 3), 'C'),
        (IVec2::new(2, 4), 'D'),
    ])
});

fn main() -> anyhow::Result<()> {
    let (_, _, p1_code, p2_code) = BufReader::new(File::open("input.txt")?).lines().try_fold(
        (
            IVec2::new(1, 1),
            IVec2::new(0, 2),
            String::new(),
            String::new(),
        ),
        |(mut p1, mut p2, mut p1_code, mut p2_code), line| -> anyhow::Result<_> {
            let line = line?;

            if !line.is_empty() {
                for c in line.chars() {
                    let dir = match c {
                        'U' => IVec2::new(0, -1),
                        'R' => IVec2::new(1, 0),
                        'D' => IVec2::new(0, 1),
                        'L' => IVec2::new(-1, 0),
                        _ => bail!("unknown direction '{c}'"),
                    };

                    p1 = (p1 + dir).clamp(IVec2::ZERO, IVec2::new(2, 2));

                    if P2_KEYPAD.contains_key(&(p2 + dir)) {
                        p2 += dir;
                    }
                }

                p1_code.push((b'1' + (p1.x + p1.y * 3) as u8) as char);
                p2_code.push(P2_KEYPAD[&p2]);
            }

            Ok((p1, p2, p1_code, p2_code))
        },
    )?;

    println!("the p1 code is {p1_code}");
    println!("the p2 code is {p2_code}");

    Ok(())
}
