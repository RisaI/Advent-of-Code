use rustc_hash::FxHashMap as HashMap;

fn split_even_digits(number: usize) -> Option<(usize, usize)> {
    match number.ilog10() + 1 {
        digits if digits % 2 == 0 => {
            let mask = 10usize.pow(digits / 2);

            Some((number / mask, number % mask))
        }
        _ => None,
    }
}

fn count_stones_after_steps(
    number: usize,
    steps: usize,
    dynamic: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if steps == 0 {
        return 1;
    }

    if let Some(v) = dynamic.get(&(number, steps)) {
        return *v;
    }

    let v = if number == 0 {
        count_stones_after_steps(1, steps - 1, dynamic)
    } else if let Some((a, b)) = split_even_digits(number) {
        count_stones_after_steps(a, steps - 1, dynamic)
            + count_stones_after_steps(b, steps - 1, dynamic)
    } else {
        count_stones_after_steps(number * 2024, steps - 1, dynamic)
    };

    dynamic.insert((number, steps), v);

    v
}

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("input.txt")?;

    let mut dynamic = HashMap::default();

    let elapsed = std::time::Instant::now();
    let stones = data
        .trim()
        .split(' ')
        .map(|v| count_stones_after_steps(v.parse::<usize>().unwrap(), 25, &mut dynamic))
        .sum::<usize>();
    let elapsed = elapsed.elapsed();

    println!(
        "{stones} stones after 25 steps in {}us",
        elapsed.as_micros()
    );

    let elapsed = std::time::Instant::now();
    let stones = data
        .trim()
        .split(' ')
        .map(|v| count_stones_after_steps(v.parse::<usize>().unwrap(), 75, &mut dynamic))
        .sum::<usize>();
    let elapsed = elapsed.elapsed();

    println!(
        "{stones} stones after 75 steps in {}us",
        elapsed.as_micros()
    );

    Ok(())
}
