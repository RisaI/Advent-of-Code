use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Read},
    ops::Index,
    path::Path,
};

use anyhow::{bail, Result};
use glam::IVec2;

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
