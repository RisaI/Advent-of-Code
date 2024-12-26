use anyhow::ensure;

fn is_triangle(sides: &[usize]) -> bool {
    sides[0] + sides[1] > sides[2]
        && sides[0] + sides[2] > sides[1]
        && sides[1] + sides[2] > sides[0]
}

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("input.txt")?
        .split('\n')
        .filter(|v| !v.is_empty())
        .map(|l| {
            l.split(' ')
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let p1_valid = data
        .iter()
        .try_fold(0, |mut state, nums| -> anyhow::Result<_> {
            ensure!(nums.len() == 3, "there should be 3 numbers per line");

            if is_triangle(nums) {
                state += 1;
            }

            Ok(state)
        })?;

    let p2_valid = data.chunks_exact(3).fold(0, |mut state, rows| {
        for i in 0..3 {
            let nums = rows.iter().map(|v| v[i]).collect::<Vec<_>>();

            if is_triangle(&nums) {
                state += 1;
            }
        }

        state
    });

    println!("there are {} valid triangles", p1_valid);
    println!("there are {} valid columnar triangles", p2_valid);

    Ok(())
}
