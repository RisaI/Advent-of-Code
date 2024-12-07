use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("data.txt")?);

    let mut area = 0;
    let mut ribbon = 0;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let mut nums = line.split('x').map(|v| v.parse::<usize>());

        let [Some(Ok(w)), Some(Ok(h)), Some(Ok(l)), None] = std::array::from_fn(|_| nums.next())
        else {
            bail!("invalid line format")
        };

        let mut sides = [w, h, l];
        sides.sort();

        let [w, h, l] = sides;
        let sides = [w * h, h * l, l * w];

        area += sides.iter().map(|s| 2 * s).sum::<usize>() + sides.iter().min().unwrap();
        ribbon += 2 * (w + h) + w * h * l;
    }

    println!("total area {area}, total ribbon {ribbon}");

    Ok(())
}
