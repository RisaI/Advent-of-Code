mod map;

pub use map::*;

pub use glam::{IVec2, Vec2};
pub use rustc_hash::{FxHashMap, FxHashSet};

pub fn manhattan(a: IVec2, b: IVec2) -> usize {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as usize
}
