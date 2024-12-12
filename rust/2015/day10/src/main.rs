fn look_and_say(mut values: &[u8]) -> Vec<u8> {
    let mut result = vec![];

    while let Some(&c) = values.first() {
        let count = values.iter().take_while(|&&v| v == c).count();

        result.extend([count as u8, c]);

        values = &values[count..];
    }

    result
}

#[test]
fn looks_and_says() {
    assert_eq!(look_and_say(&[1]), &[1, 1]);
    assert_eq!(look_and_say(&[1, 1]), &[2, 1]);
    assert_eq!(look_and_say(&[2, 1]), &[1, 2, 1, 1]);
    assert_eq!(look_and_say(&[1, 2, 1, 1]), &[1, 1, 1, 2, 2, 1]);
    assert_eq!(look_and_say(&[1, 1, 1, 2, 2, 1]), &[3, 1, 2, 2, 1, 1]);
}

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("data.txt")?;

    let data = data
        .as_bytes()
        .iter()
        .map(|b| *b - b'0')
        .collect::<Vec<_>>();

    let start = std::time::Instant::now();

    for len in [40, 50] {
        let mut data = data.clone();

        for _ in 0..len {
            data = look_and_say(&data);
        }

        println!(
            "after {len} iterations, the string is {} digits long",
            data.len()
        );
    }

    println!("took {}ms", start.elapsed().as_millis());

    Ok(())
}
