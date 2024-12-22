use clap::Parser as ClapParser;
use memchr::memchr2;
use winnow::{
    ascii::digit1,
    combinator::{alt, empty, separated_pair, terminated},
    error::ContextError,
    prelude::*,
};

#[derive(ClapParser)]
struct Options {
    #[arg(long)]
    pub no_conditionals: bool,
}

enum Instruction {
    Do,
    Dont,
    Mul(usize, usize),
}

fn parse_instruction<'a>() -> impl Parser<&'a str, Instruction, ContextError> {
    alt((
        ("do()", empty.map(|_| Instruction::Do)),
        ("don't()", empty.map(|_| Instruction::Dont)),
        (
            "mul(",
            terminated(
                separated_pair(digit1.parse_to::<usize>(), ',', digit1.parse_to::<usize>()),
                ')',
            )
            .map(|(a, b)| Instruction::Mul(a, b)),
        ),
    ))
    .map(|(_, v)| v)
}

fn main() -> anyhow::Result<()> {
    let opts = Options::parse();
    let data = std::fs::read_to_string("data.txt")?;

    let mut window = data.as_str();
    let mut sum = 0;
    let mut enabled = true;
    let mut parser = parse_instruction();

    while !window.is_empty() {
        let Some(candidate) = memchr2(b'd', b'm', window.as_bytes()) else {
            break;
        };

        window = &window[candidate..];

        if let Ok(instruction) = parser.parse_next(&mut window) {
            match instruction {
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = opts.no_conditionals,
                Instruction::Mul(a, b) if enabled => sum += a * b,
                _ => { /* noop */ }
            }
        } else {
            window = &window[1..]
        }
    }

    println!("{sum}");

    Ok(())
}
