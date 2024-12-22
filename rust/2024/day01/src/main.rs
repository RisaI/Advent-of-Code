use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let start = std::time::Instant::now();
    let reader = BufReader::new(File::open("data.txt")?);

    let mut left_list = BinaryHeap::<usize>::new();
    let mut right_list = BinaryHeap::<usize>::new();

    let mut left_items = HashSet::new();
    let mut right_occurrences: HashMap<usize, usize> = HashMap::<usize, usize>::new();

    for line in reader.lines() {
        let line = line?;

        let mut split = line.split("   ");

        let [Some(left), Some(right)] =
            std::array::from_fn(|_| split.next().and_then(|v| v.parse().ok()))
        else {
            bail!("failed to parse input")
        };

        left_list.push(left);
        right_list.push(right);

        left_items.insert(left);
        *right_occurrences.entry(right).or_default() += 1;
    }

    let result = std::iter::from_fn(|| left_list.pop())
        .zip(std::iter::from_fn(|| right_list.pop()))
        .map(|(l, r)| l.abs_diff(r))
        .sum::<usize>();

    println!("the list diff is {result}");

    let similarity = left_items
        .iter()
        .map(|i| right_occurrences.get(i).copied().unwrap_or_default() * *i)
        .sum::<usize>();

    println!("similarity score {similarity}");

    let elapsed = start.elapsed();
    println!("took {}us", elapsed.as_micros());

    Ok(())
}
