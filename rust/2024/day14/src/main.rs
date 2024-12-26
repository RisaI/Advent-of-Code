use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use glam::{IVec2, Vec2};
use winnow::{
    ascii::digit1,
    combinator::{opt, preceded, separated_pair},
    error::InputError,
    Parser,
};

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

struct Arena {
    extents: IVec2,
}

impl Arena {
    pub fn new(extents: IVec2) -> Self {
        assert!(
            extents.x % 2 == 1 && extents.y % 2 == 1,
            "extents must be odd"
        );

        Self { extents }
    }

    pub fn quadrant(&self, pos: IVec2) -> Option<usize> {
        use std::cmp::Ordering::*;

        if pos.x < 0 || pos.y < 0 || pos.x >= self.extents.x || pos.y >= self.extents.y {
            return None;
        }

        match (
            pos.x.cmp(&(self.extents.x / 2)),
            pos.y.cmp(&(self.extents.y / 2)),
        ) {
            (_, Equal) | (Equal, _) => None,
            (Less, Less) => Some(0),
            (Less, Greater) => Some(1),
            (Greater, Less) => Some(2),
            (Greater, Greater) => Some(3),
        }
    }

    pub fn mod_pos(&self, pos: IVec2) -> IVec2 {
        IVec2::new(modulo(pos.x, self.extents.x), modulo(pos.y, self.extents.y))
    }
}

struct Robot {
    start_pos: IVec2,
    velocity: IVec2,
}

impl Robot {
    pub fn position_after_steps(&self, steps: i32, map: &Arena) -> IVec2 {
        map.mod_pos(self.start_pos + steps * self.velocity)
    }
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn signed_digit<'a>() -> impl Parser<&'a str, i32, InputError<&'a str>> {
            (opt('-'), digit1.parse_to::<i32>()).map(|(s, v)| if s.is_some() { -v } else { v })
        }

        let ((px, py), (vx, vy)) = separated_pair(
            preceded(
                "p=",
                separated_pair(digit1.parse_to::<i32>(), ',', digit1.parse_to::<i32>()),
            ),
            ' ',
            preceded("v=", separated_pair(signed_digit(), ',', signed_digit())),
        )
        .parse(s)
        .map_err(|e| anyhow::format_err!("{e}"))?;

        Ok(Self {
            start_pos: IVec2::new(px, py),
            velocity: IVec2::new(vx, vy),
        })
    }
}

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("input.txt")?);

    let map = Arena::new(IVec2::new(101, 103));

    let robots =
        reader
            .lines()
            .try_fold(vec![], |mut robots, line| -> anyhow::Result<Vec<Robot>> {
                let line = line?;

                robots.push(line.parse()?);

                Ok(robots)
            })?;

    let security_score = robots
        .iter()
        .fold(vec![0; 4], |mut score, robot| {
            let pos = robot.position_after_steps(100, &map);

            if let Some(quadrant) = map.quadrant(pos) {
                score[quadrant] += 1;
            }

            score
        })
        .into_iter()
        .product::<i32>();

    println!("the security score after 100s is {security_score}");

    for secs in 1.. {
        let positions = robots
            .iter()
            .map(|r| r.position_after_steps(secs, &map))
            .collect::<Vec<_>>();

        let mean = positions
            .iter()
            .map(|v| Vec2::new(v.x as f32, v.y as f32))
            .sum::<Vec2>()
            / positions.len() as f32;
        let stdev = (positions
            .iter()
            .map(|p| (Vec2::new(p.x as f32, p.y as f32) - mean).length_squared())
            .sum::<f32>()
            / positions.len() as f32)
            .sqrt();

        if stdev < 30. {
            println!("{secs}s -> {mean} +- {stdev:.3}");
            break;
        }
    }

    Ok(())
}
