use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("data.txt")?);

    let mut code_minus_bytes = 0;
    let mut code_minus_code = 0;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        code_minus_bytes += line.len();

        let mut chars = line.chars().peekable();

        // Skip leading "
        chars.next();

        while let Some(ch) = chars.next() {
            match (ch, chars.peek()) {
                ('"', None) => continue,
                ('\\', Some('\\')) | ('\\', Some('"')) => {
                    chars.next();
                }
                ('\\', Some('x')) => {
                    let next = chars.clone().take(3).collect::<Vec<_>>();

                    if next.len() == 3 && next[1].is_ascii_hexdigit() && next[2].is_ascii_hexdigit()
                    {
                        for _ in 0..3 {
                            chars.next();
                        }
                    }
                }
                _ => (),
            }

            code_minus_bytes -= 1;
        }

        code_minus_code += 2 + line
            .chars()
            .map(|c| match c {
                '\\' | '"' => 2,
                _ => 1,
            })
            .sum::<usize>()
            - line.chars().count();
    }

    println!("the difference between code chars and in-memory chars is {code_minus_bytes}");
    println!("the difference between code representations is {code_minus_code}");

    Ok(())
}
