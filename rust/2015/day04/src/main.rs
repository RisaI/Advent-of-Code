use std::io::Write;

use md5::{Digest, Md5};

fn main() -> anyhow::Result<()> {
    let mut hasher = Md5::new();
    let key = "yzbqklnj";

    for i in 0.. {
        write!(hasher, "{key}{i}")?;
        let result = Digest::finalize_reset(&mut hasher);

        if result[0] == 0 && result[1] == 0 && result[2] < 16 {
            if result[2] == 0 {
                println!("6 zeroes {i}");
                break;
            } else {
                println!("5 zeroes {i}");
            }
        }
    }

    Ok(())
}
