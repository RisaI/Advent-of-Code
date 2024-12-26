fn pair_bracket(data: &str) -> &str {
    let mut depth = 0;

    for (idx, c) in data.chars().enumerate() {
        match c {
            '{' => {
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &data[0..=idx];
                }
            }
            _ => (),
        }
    }

    data
}

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("input.txt")?;

    {
        let mut data = &data[..];
        let mut sum: isize = 0;

        while !data.is_empty() {
            let digits = data
                .chars()
                .take_while(|ch| ch.is_numeric() || *ch == '-')
                .count();

            if digits > 0 {
                if let Ok(v) = data[0..digits].parse::<isize>() {
                    sum += v;
                }

                data = &data[digits..];
            } else if let Some(idx) = data.find(|ch: char| ch.is_numeric() || ch == '-') {
                data = &data[idx..];
            } else {
                break;
            }
        }

        println!("the sum is {sum}");
    }

    {
        let mut data = &data[..];
        let mut sum: isize = 0;

        while let Some(idx) = data.find(|ch: char| ch.is_ascii_digit() || ch == '-' || ch == '{') {
            data = &data[idx..];

            match data.chars().next().unwrap() {
                '{' => {
                    let mut bracket = pair_bracket(data);
                    let total_len = bracket.len();
                    bracket = &bracket[1..];

                    while !bracket.is_empty() {
                        let (region, skip_to) = match bracket.find('{') {
                            Some(child_idx) => (
                                child_idx,
                                child_idx + pair_bracket(&bracket[child_idx..]).len(),
                            ),
                            None => (bracket.len(), bracket.len()),
                        };

                        if bracket[0..region].contains(":\"red\"") {
                            // Skip bracket
                            data = &data[total_len..];
                            break;
                        }

                        bracket = &bracket[skip_to..];
                    }
                }
                '-' | '0'..='9' => {
                    let digits = data
                        .chars()
                        .skip(1)
                        .take_while(|c| c.is_ascii_digit())
                        .count()
                        + 1;

                    if let Ok(v) = data[0..digits].parse::<isize>() {
                        sum += v;
                    }

                    data = &data[digits..];

                    continue;
                }
                _ => (),
            }

            data = &data[1..]
        }

        println!("the sum without red is {sum}");
    }

    Ok(())
}
