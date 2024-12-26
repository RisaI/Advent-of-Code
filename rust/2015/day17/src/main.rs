use anyhow::Context;

fn partitions(value: usize, containers: &[usize], budget: usize) -> usize {
    if containers.is_empty() || budget == 0 {
        return 0;
    }

    (match containers[0].cmp(&value) {
        std::cmp::Ordering::Greater => 0,
        std::cmp::Ordering::Equal => 1,
        std::cmp::Ordering::Less => partitions(value - containers[0], &containers[1..], budget - 1),
    }) + partitions(value, &containers[1..], budget)
}

fn main() -> anyhow::Result<()> {
    let containers = std::fs::read_to_string("input.txt")?
        .split('\n')
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    println!(
        "{} partitions",
        partitions(150, &containers, containers.len())
    );

    let (num_containers, combinations) = (1..containers.len())
        .map(|c| (c, partitions(150, &containers, c)))
        .find(|(_, v)| *v > 0)
        .context("no solution for p2 found, something must be wrong")?;

    println!(
        "{} partitions with {} containers",
        combinations, num_containers
    );

    Ok(())
}
