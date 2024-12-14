use std::{
    array::from_fn,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use nalgebra::{vector, DVector};
use winnow::{
    ascii::{alpha1, digit1},
    combinator::{opt, separated, separated_pair},
    error::InputError,
    Parser,
};

struct Ingredient([isize; 5]);

impl FromStr for Ingredient {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, _, ingredients): (&str, &str, Vec<(&str, isize)>) = (
            alpha1::<_, InputError<&str>>,
            ": ",
            separated(
                5..=5,
                separated_pair(
                    alpha1,
                    ' ',
                    (opt('-'), digit1.parse_to::<isize>())
                        .map(|(s, v)| if s.is_some() { -v } else { v }),
                ),
                ", ",
            ),
        )
            .parse(s)
            .map_err(|e| anyhow::format_err!("{e}"))?;

        Ok(Self(from_fn(|i| ingredients[i].1)))
    }
}

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("data.txt")?);

    let ingredients = reader.lines().try_fold(
        vec![],
        |mut prev, line| -> anyhow::Result<Vec<Ingredient>> {
            let line = line?;

            if !line.is_empty() {
                prev.push(line.parse()?);
            }

            Ok(prev)
        },
    )?;

    let vecs = ingredients
        .iter()
        .map(|i| DVector::from_iterator(5, i.0))
        .collect::<Vec<_>>();

    let mat = nalgebra::DMatrix::from_columns(&vecs);

    let spoons = 100;
    let mut max = 0;
    let mut max_p2 = 0;

    // FIXME: hardcoded and ugly
    for i in 0..=spoons {
        for j in 0..=(spoons - i) {
            for k in 0..=(spoons - i - j) {
                let l = spoons - i - j - k;

                let v = &mat * vector![i, j, k, l];
                let score = v.into_iter().map(|&v| v.max(0)).take(4).product::<isize>();

                if score > max {
                    max = score;
                }

                if v[4] == 500 && score > max_p2 {
                    max_p2 = score;
                }
            }
        }
    }

    println!("max score = {max}");
    println!("max (500 cal) score = {max_p2}");

    Ok(())
}
