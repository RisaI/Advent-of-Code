use std::{
    fmt::Write,
    fs::File,
    io::{BufRead, BufReader, Cursor, Read},
    ops::Index,
    path::Path,
};

use anyhow::{bail, Result};
use glam::IVec2;
use rustc_hash::FxHashMap;

static MOVEMENT_DIRECTIONS: &[IVec2] = &[
    IVec2::new(1, 0),
    IVec2::new(0, 1),
    IVec2::new(-1, 0),
    IVec2::new(0, -1),
];

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Map2D<T> {
    width: usize,
    data: Box<[T]>,
}

pub trait MapConstructParam {
    fn from(ch: char, x: usize, y: usize) -> Self;
}

impl MapConstructParam for char {
    fn from(ch: char, _: usize, _: usize) -> Self {
        ch
    }
}

impl MapConstructParam for (char, IVec2) {
    fn from(ch: char, x: usize, y: usize) -> Self {
        (ch, IVec2::new(x as i32, y as i32))
    }
}

impl<T> Map2D<T> {
    pub fn read_file<P>(path: impl AsRef<Path>, f: impl FnMut(P) -> T) -> Result<Self>
    where
        P: MapConstructParam,
    {
        Self::read(File::open(path)?, f)
    }

    pub fn read_str<P: MapConstructParam>(data: &str, f: impl FnMut(P) -> T) -> Result<Self> {
        Self::read(Cursor::new(data), f)
    }

    pub fn read<P: MapConstructParam>(
        reader: impl Read,
        mut f: impl FnMut(P) -> T,
    ) -> Result<Self> {
        let mut data = vec![];

        let reader = BufReader::new(reader);
        let mut width = 0;

        for (y, line) in reader.lines().enumerate() {
            let line = line?;

            if line.is_empty() {
                break;
            }

            if width == 0 {
                width = line.len();
            }

            if line.len() != width {
                bail!("inconsistent line width");
            }

            data.extend(line.chars().enumerate().map(|(x, ch)| f(P::from(ch, x, y))));
        }

        anyhow::ensure!(
            data.len() % width == 0,
            "data size is inconsistent with line width"
        );

        Ok(Self {
            width,
            data: data.into_boxed_slice(),
        })
    }

    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self {
            width,
            data: (0..(width * height)).map(|_| T::default()).collect(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width()
    }

    pub fn get(&self, pos: IVec2) -> Option<&T> {
        let x: usize = pos.x.try_into().ok()?;
        let y: usize = pos.y.try_into().ok()?;

        if x >= self.width() || y >= self.height() {
            return None;
        }

        self.data.get(x + y * self.width())
    }

    pub fn set(&mut self, IVec2 { x, y }: IVec2, v: T) {
        let [Ok(x), Ok(y)] = [x, y].map(usize::try_from) else {
            return;
        };

        if x >= self.width() || y >= self.height() {
            return;
        }

        self.data[x + y * self.width()] = v;
    }

    pub fn find<'a>(
        &'a self,
        mut f: impl (FnMut(IVec2, &T) -> bool) + 'a,
    ) -> impl Iterator<Item = (IVec2, &'a T)> + 'a {
        self.data.iter().enumerate().filter_map(move |(i, v)| {
            let pos = IVec2::from(((i % self.width()) as i32, (i / self.width()) as i32));

            if f(pos, v) {
                Some((pos, v))
            } else {
                None
            }
        })
    }
}

pub struct AStarOptions<'a> {
    pub from: IVec2,
    pub to: IVec2,
    pub directions: &'a [IVec2],
    pub max_steps: Option<usize>,
}

impl AStarOptions<'_> {
    pub fn new(from: IVec2, to: IVec2) -> Self {
        Self {
            from,
            to,
            directions: MOVEMENT_DIRECTIONS,
            max_steps: None,
        }
    }
}

impl Map2D<bool> {
    pub fn a_star(
        &'_ self,
        AStarOptions {
            from,
            to,
            directions,
            max_steps,
        }: AStarOptions<'_>,
    ) -> Option<Vec<IVec2>> {
        #[derive(Default)]
        struct PointInfo {
            pub steps: usize,
            pub cost_estimate: usize,
            pub prev_tile: Option<IVec2>,
        }

        let mut queue = vec![];
        let mut known = FxHashMap::<IVec2, PointInfo>::default();
        queue.push(from);
        known.insert(from, PointInfo::default());

        while !queue.is_empty() {
            queue.sort_by(|a, b| {
                known
                    .get(a)
                    .unwrap()
                    .cost_estimate
                    .cmp(&known.get(b).unwrap().cost_estimate)
                    .reverse()
            });

            let pos = queue.pop()?;
            let steps = known.get(&pos).unwrap().steps;

            if let Some(true) = max_steps.map(|v| v < steps) {
                continue;
            }

            // println!("{pos}");

            if pos == to {
                let mut cursor = pos;
                return Some(
                    std::iter::once(pos)
                        .chain(std::iter::from_fn::<IVec2, _>(|| {
                            match known.get(&cursor) {
                                Some(p) => {
                                    cursor = p.prev_tile.unwrap_or(cursor);

                                    p.prev_tile
                                }
                                None => None,
                            }
                        }))
                        .collect(),
                );
            }

            for d in directions {
                let next = pos + d;

                if !matches!(self.get(next), Some(false)) {
                    continue;
                }

                match known.get(&next) {
                    None => (),
                    Some(PointInfo {
                        steps: prev_steps, ..
                    }) if *prev_steps > steps + 1 => (),
                    _ => continue,
                }

                queue.push(next);
                known.insert(
                    next,
                    PointInfo {
                        steps: steps + 1,
                        cost_estimate: steps
                            + 1
                            + ((to.x - next.x).abs() + (to.y - next.y).abs()) as usize,
                        prev_tile: Some(pos),
                    },
                );
            }
        }
        None
    }
}

impl std::fmt::Display for Map2D<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            if y > 0 {
                f.write_char('\n')?;
            }

            for x in 0..self.width() {
                f.write_char(
                    if let Some(true) = self.get(IVec2::new(x as i32, y as i32)) {
                        '#'
                    } else {
                        '.'
                    },
                )?;
            }
        }

        Ok(())
    }
}

impl<T> AsRef<[T]> for Map2D<T> {
    fn as_ref(&self) -> &[T] {
        &self.data
    }
}

impl<T> Index<IVec2> for Map2D<T> {
    type Output = T;

    fn index(&self, index: IVec2) -> &Self::Output {
        &self.data[index.x as usize + index.y as usize * self.width]
    }
}
