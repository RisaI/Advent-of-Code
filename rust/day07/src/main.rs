use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ops {
    Sum,
    Mul,
    Con,
}

impl Ops {
    const ALL: [Self; 3] = [Self::Sum, Self::Mul, Self::Con];
}

fn number_ends_with(number: isize, suffix: isize) -> Option<isize> {
    let mut mask = 10;
    loop {
        if (number % mask) != (suffix % mask) {
            return None;
        }

        if suffix / mask == 0 {
            return Some(mask);
        }

        mask = mask.checked_mul(10)?;
    }
}

#[test]
fn ends_with_just_werks() {
    assert_eq!(number_ends_with(123, 456), None);
    assert_eq!(number_ends_with(123, 3), Some(10));
    assert_eq!(number_ends_with(1337, 37), Some(100));
}

fn check_recursive(result: isize, values: &[isize], no_con: bool) -> bool {
    let Some(&value) = values.last() else {
        return false;
    };

    if values.len() == 1 {
        return value == result;
    }

    let ops = match no_con {
        true => &Ops::ALL[0..2],
        false => &Ops::ALL,
    };

    for op in ops {
        let Some(next) = (match op {
            Ops::Sum if value < result => Some(result - value),
            Ops::Mul if result % value == 0 => Some(result / value),
            Ops::Con => number_ends_with(result, value).map(|order| result / order),
            _ => None,
        }) else {
            continue;
        };

        if check_recursive(next, &values[0..(values.len() - 1)], no_con) {
            return true;
        }
    }

    false
}

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("data.txt")?);

    let mut sums = [0, 0];

    let start = std::time::Instant::now();

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let Some((result, values)) = line.split_once(": ") else {
            continue;
        };

        let result = result.parse::<isize>()?;
        let values: Vec<isize> = values
            .split(' ')
            .map(|v| v.parse())
            .collect::<Result<_, _>>()?;

        for (no_con, sum) in [false, true].into_iter().zip(sums.iter_mut()) {
            if check_recursive(result, &values, no_con) {
                *sum += result;
            }
        }
    }

    println!("calibration sum without con = {}", sums[1]);
    println!("calibration sum = {}", sums[0]);
    println!("finished in {}us", start.elapsed().as_micros());

    Ok(())
}
