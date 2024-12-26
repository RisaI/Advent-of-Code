use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("input.txt")?
        .trim()
        .parse::<usize>()?;

    let p1 = {
        let mut array = vec![0; data / 20];

        for i in 0..array.len() {
            let v = i + 1;

            for j in (i..array.len()).step_by(v) {
                array[j] += v * 10;
            }
        }

        1 + array
            .iter()
            .position(|x| *x >= data)
            .context("no solution found")?
    };

    println!("first house above {} is {} (p1)", data, p1);

    let p2 = {
        let mut array = vec![0; data / 11];

        for i in 0..array.len() {
            let v = i + 1;

            for j in (i..array.len()).step_by(v).take(50) {
                array[j] += v * 11;
            }
        }

        1 + array
            .iter()
            .position(|x| *x >= data)
            .context("no solution found")?
    };

    println!("first house above {} is {} (p2)", data, p2);

    Ok(())
}
