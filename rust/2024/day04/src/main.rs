use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use smallvec::SmallVec;

static P1_TARGETS: &[&[u8]] = &[b"XMAS", b"SAMX"];
static P2_TARGETS: &[&[u8]] = &[b"MAS", b"SAM"];

fn main() -> anyhow::Result<()> {
    let data = BufReader::new(File::open("input.txt")?)
        .lines()
        .map(|v| v.map(|v| v.into_bytes()))
        .collect::<Result<Vec<_>, _>>()?;

    let rows = data.len();
    let cols = data[0].len();

    // Part one
    let p1_length = P1_TARGETS[0].len();
    let p2_length = P2_TARGETS[0].len();
    let mut p1_sum = 0;
    let mut p2_sum = 0;

    let mut candidates = smallvec::SmallVec::<[[u8; P1_TARGETS[0].len()]; 4]>::with_capacity(4);

    for col_idx in 0..cols {
        for row_idx in 0..rows {
            // Part one
            candidates.clear();

            if col_idx <= cols - p1_length {
                candidates.push(std::array::from_fn(|k| data[row_idx][col_idx + k]));
            }

            if row_idx <= rows - p1_length {
                candidates.push(std::array::from_fn(|k| data[row_idx + k][col_idx]));
            }

            if col_idx <= cols - p1_length && row_idx <= rows - p1_length {
                candidates.push(std::array::from_fn(|k| data[row_idx + k][col_idx + k]));
            }

            if row_idx <= rows - p1_length && col_idx >= p1_length - 1 {
                candidates.push(std::array::from_fn(|k| data[row_idx + k][col_idx - k]));
            }

            p1_sum += candidates
                .iter()
                .filter(|c| P1_TARGETS.contains(&c.as_slice()))
                .count();

            // Part two
            if col_idx <= cols - p2_length && row_idx <= rows - p2_length {
                let candidates: [SmallVec<[u8; P2_TARGETS[0].len()]>; 2] = [
                    (0..p2_length)
                        .map(|k| data[row_idx + k][col_idx + k])
                        .collect(),
                    (0..p2_length)
                        .map(|k| data[row_idx + k][col_idx + p2_length - 1 - k])
                        .collect(),
                ];

                if candidates
                    .iter()
                    .all(|c| P2_TARGETS.contains(&c.as_slice()))
                {
                    p2_sum += 1
                }
            }
        }
    }

    println!("{p1_sum} XMAS occurrences");
    println!("{p2_sum} X MAS occurrences");

    Ok(())
}
