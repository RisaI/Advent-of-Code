fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("data.txt")?;

    let floor = data.chars().map(|ch| match ch {
        '(' => 1,
        ')' => -1,
        _ => 0,
    });

    let final_floor = floor.clone().sum::<isize>();

    let Some((basement_index, _)) = floor
        .scan(0, |acc, next| {
            *acc += next;

            Some(*acc)
        })
        .enumerate()
        .find(|(_, floor)| *floor < 0)
    else {
        anyhow::bail!("basement never reached");
    };

    println!("final floor: {final_floor}");
    println!("basement index: {}", basement_index + 1);

    Ok(())
}
