use core::str;

fn valid_password(pass: &[u8]) -> bool {
    pass.windows(3)
        .any(|w| w[2] == w[0] + 2 && w[1] == w[0] + 1)
        && pass.iter().all(|&c| c != b'i' && c != b'l' && c != b'o')
        && pass
            .windows(2)
            .enumerate()
            .any(|(i, w)| w[0] == w[1] && pass[(i + 2)..].windows(2).any(|w| w[0] == w[1]))
}

fn next_password(pass: &mut [u8]) -> &[u8] {
    loop {
        for i in (0..pass.len()).rev() {
            pass[i] += 1;

            if pass[i] <= b'z' {
                break;
            } else {
                pass[i] = b'a';
            }
        }

        if valid_password(pass) {
            break;
        }
    }

    pass
}

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("input.txt")?;
    let mut bytes: Box<[u8]> = data.trim().as_bytes().into();

    println!(
        "next pass is {}",
        str::from_utf8(next_password(&mut bytes)).unwrap()
    );

    println!(
        "and then, the next pass is {}",
        str::from_utf8(next_password(&mut bytes)).unwrap()
    );

    Ok(())
}
