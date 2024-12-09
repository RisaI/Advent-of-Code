type FileSystem = Vec<(Option<usize>, u8)>;

fn parse_fs(text: &str) -> FileSystem {
    let mut fs: Vec<(Option<usize>, u8)> = vec![];

    for (idx, char) in text.chars().enumerate() {
        let len = char as u8 - b'0';

        if idx % 2 == 0 {
            fs.push((Some(idx / 2), len));
        } else {
            fs.push((None, len));
        }
    }

    fs
}

fn compact(fs: &mut FileSystem) {
    let mut file_idx = fs.len() - 1;
    loop {
        let (Some(file_id), mut space) = fs[file_idx] else {
            file_idx -= 1;
            continue;
        };

        let og_space = space;

        while space > 0 {
            let Some((free_idx, free_block)) = fs
                .iter_mut()
                .take(file_idx)
                .enumerate()
                .find(|(_, v)| v.0.is_none())
            else {
                break;
            };

            if free_block.1 <= space {
                free_block.0 = Some(file_id);
                space -= free_block.1;
            } else {
                let remaining = free_block.1 - space;

                *free_block = (Some(file_id), space);
                fs.insert(free_idx + 1, (None, remaining));
                file_idx += 1;
                space = 0;
            }
        }

        if space > 0 {
            fs[file_idx].1 = space;
            fs.insert(file_idx + 1, (None, og_space - space));
            break;
        } else {
            fs[file_idx] = (None, og_space);
        }

        if file_idx == 0 {
            break;
        } else {
            file_idx -= 1;
        }
    }
}

#[test]
fn compact_just_werks() {
    let mut fs = parse_fs("2333133121414131402");

    compact(&mut fs);

    assert_eq!(
        fs.iter()
            .enumerate()
            .rev()
            .find(|(_, v)| v.0.is_some())
            .unwrap()
            .0
            + 1,
        fs.iter()
            .enumerate()
            .find(|(_, v)| v.0.is_none())
            .unwrap()
            .0,
        "first empty block should be after last non-empty block"
    );
}

fn compact_non_fragmented(fs: &mut FileSystem) {
    for gap_idx in 0.. {
        if gap_idx >= fs.len() {
            break;
        }

        let (None, gap_size) = fs[gap_idx] else {
            continue;
        };

        let Some((file_idx, &(Some(_), file_size))) = fs
            .iter()
            .enumerate()
            .skip(gap_idx + 1)
            .rev()
            .find(|(_, &(id, size))| id.is_some() && size <= gap_size)
        else {
            continue;
        };

        let remainder = gap_size - file_size;

        if remainder > 0 {
            fs[gap_idx].1 = file_size;
            fs.swap(gap_idx, file_idx);
            fs.insert(gap_idx + 1, (None, remainder));
        } else {
            fs.swap(gap_idx, file_idx);
        }
    }
}

fn checksum(fs: &FileSystem) -> usize {
    fs.iter()
        .fold((0, 0), |(block_idx, mut sum), (file_id, file_size)| {
            if let Some(file_id) = *file_id {
                for i in 0..(*file_size) {
                    sum += (block_idx + i as usize) * file_id;
                }
            };

            (block_idx + *file_size as usize, sum)
        })
        .1
}

#[test]
fn checksum_just_werks() {
    let mut fs = parse_fs("2333133121414131402");

    compact(&mut fs);

    assert_eq!(checksum(&fs), 1928);

    let mut fs = parse_fs("2333133121414131402");

    println!("{fs:?}");
    compact_non_fragmented(&mut fs);
    println!("{fs:?}");

    assert_eq!(checksum(&fs), 2858)
}

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("data.txt")?;

    let mut fs = parse_fs(&data);

    compact(&mut fs);

    println!("checksum: {}", checksum(&fs));

    let mut fs = parse_fs(&data);

    compact_non_fragmented(&mut fs);

    println!("non-fragmented checksum: {}", checksum(&fs));

    Ok(())
}
