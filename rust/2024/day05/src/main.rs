use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::bail;

fn is_order_correct(pages: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> bool {
    for (idx, val) in pages.iter().enumerate() {
        let Some(after) = rules.get(val) else {
            continue;
        };

        for prev in pages.iter().take(idx) {
            if after.contains(prev) {
                return false;
            }
        }
    }

    true
}

fn order_pages(pages: &mut [usize], rules: &HashMap<usize, HashSet<usize>>) {
    'correction: loop {
        for (idx, val) in pages.iter().enumerate() {
            let Some(rules) = rules.get(val) else {
                continue;
            };

            for (prev_idx, prev_val) in pages.iter().enumerate().take(idx) {
                if rules.contains(prev_val) {
                    pages.swap(prev_idx, idx);
                    continue 'correction;
                }
            }
        }

        break;
    }
}

fn main() -> anyhow::Result<()> {
    let mut lines = BufReader::new(File::open("input.txt")?).lines();

    let mut rules = HashMap::<usize, HashSet<usize>>::new();

    for line in &mut lines {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let Some((before, after)) = line.split_once('|') else {
            bail!("format error: {line}");
        };

        let [before, after]: [usize; 2] = [before.parse()?, after.parse()?];

        rules.entry(before).or_default().insert(after);
    }

    let mut correct_sum = 0;
    let mut corrected_sum = 0;

    for line in &mut lines {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let mut pages = line
            .split(',')
            .map(|v| v.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;

        if is_order_correct(&pages, &rules) {
            correct_sum += pages[pages.len() / 2];
        } else {
            order_pages(&mut pages, &rules);

            corrected_sum += pages[pages.len() / 2];
        }
    }

    println!("{correct_sum} correct lines middle-sum");
    println!("{corrected_sum} corrected lines middle-sum");

    Ok(())
}
