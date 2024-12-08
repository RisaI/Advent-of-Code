use anyhow::bail;
use fxhash::{FxHashMap, FxHashSet};

type Prec = i32;
type Vec2 = glam::IVec2;

fn main() -> anyhow::Result<()> {
    let start = std::time::Instant::now();

    let data = std::fs::read_to_string("data.txt")?;
    let data = data.trim();

    let rows = data.split('\n').count() as Prec;
    let Some(cols) = data.split('\n').map(|v| v.len() as Prec).next() else {
        bail!("input is empty");
    };

    let out_of_bounds = |v: Vec2| v[0] < 0 || v[1] < 0 || v[0] >= cols || v[1] >= rows;

    let mut antennas = FxHashMap::<char, Vec<Vec2>>::default();
    let mut resonances_p1 = FxHashSet::<Vec2>::default();
    let mut resonances_p2 = FxHashSet::<Vec2>::default();

    for (row, line) in data.split('\n').enumerate() {
        if line.is_empty() {
            break;
        }

        for (col, ch) in line.chars().enumerate() {
            if ch == '.' {
                continue;
            }

            let pos = Vec2::from([col as Prec, row as Prec]);

            if let Some(others) = antennas.get(&ch) {
                for other in others {
                    let delta = *other - pos;

                    resonances_p1.extend(
                        [*other + delta, pos - delta]
                            .into_iter()
                            .filter(|v| !out_of_bounds(*v)),
                    );

                    resonances_p2.extend(
                        (0..)
                            .map(|v| *other + delta * v)
                            .take_while(|p| !out_of_bounds(*p)),
                    );

                    resonances_p2.extend(
                        (0..)
                            .map(|v| pos - delta * v)
                            .take_while(|p| !out_of_bounds(*p)),
                    );
                }
            }

            antennas.entry(ch).or_default().push(pos);
        }
    }

    println!("{} resonances (p1)", resonances_p1.len());
    println!("{} resonances (p2)", resonances_p2.len());

    println!("finished in {}us", start.elapsed().as_micros());

    Ok(())
}
