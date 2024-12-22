use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use winnow::{
    ascii::digit1,
    combinator::{fail, separated_pair},
    error::InputError,
    token::{one_of, take},
    Parser,
};

#[derive(Default)]
pub struct Computer {
    registers: [usize; 2],
}

impl Computer {
    pub fn apply_instruction(&mut self, instruction: Instruction) -> Option<isize> {
        let register = match instruction.register() {
            Some(Register::A) => &mut self.registers[0],
            Some(Register::B) => &mut self.registers[1],
            None => {
                return match instruction {
                    Instruction::Jmp(by) => Some(by),
                    _ => None,
                }
            }
        };

        match instruction {
            Instruction::Hlf(_) => *register /= 2,
            Instruction::Tpl(_) => *register *= 3,
            Instruction::Inc(_) => *register += 1,
            Instruction::Jie(_, by) => {
                if *register % 2 == 0 {
                    return Some(by);
                }
            }
            Instruction::Jio(_, by) => {
                if *register == 1 {
                    return Some(by);
                }
            }
            _ => (),
        }

        None
    }

    pub fn a_val(&self) -> usize {
        self.registers[0]
    }

    pub fn b_val(&self) -> usize {
        self.registers[1]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Register {
    A = 0,
    B = 1,
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, isize),
    Jio(Register, isize),
}

impl Instruction {
    pub fn register(&self) -> Option<Register> {
        match self {
            Instruction::Hlf(register)
            | Instruction::Tpl(register)
            | Instruction::Inc(register)
            | Instruction::Jie(register, _)
            | Instruction::Jio(register, _) => Some(*register),
            Instruction::Jmp(_) => None,
        }
    }
}

fn register_parser<'a>() -> impl Parser<&'a str, Register, InputError<&'a str>> {
    one_of(['a', 'b']).map(|v: char| match v {
        'a' => Register::A,
        'b' => Register::B,
        _ => unreachable!(),
    })
}

fn delta_parser<'a>() -> impl Parser<&'a str, isize, InputError<&'a str>> {
    (one_of(['+', '-']), digit1.parse_to::<isize>())
        .map(|(sign, v)| if sign == '+' { v } else { -v })
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        winnow::combinator::dispatch!(take(4usize);
            "hlf " => register_parser().map(Instruction::Hlf),
            "tpl " => register_parser().map(Instruction::Tpl),
            "inc " => register_parser().map(Instruction::Inc),
            "jmp " => delta_parser().map(Instruction::Jmp),
            "jie " => separated_pair(register_parser(), ", ", delta_parser()).map(|(r, d)| Instruction::Jie(r, d)),
            "jio " => separated_pair(register_parser(), ", ", delta_parser()).map(|(r, d)| Instruction::Jio(r, d)),
            _ => fail
        )
        .parse(s)
        .map_err(|e| anyhow::format_err!("{e}"))
    }
}

fn main() -> anyhow::Result<()> {
    let program = BufReader::new(File::open("data.txt")?).lines().try_fold(
        vec![],
        |mut state, line| -> anyhow::Result<_> {
            let line = line?;

            if !line.is_empty() {
                state.push(line.parse::<Instruction>()?);
            }

            Ok(state)
        },
    )?;

    for a in [0, 1] {
        let mut puter = Computer { registers: [a, 0] };
        let mut cursor = 0;

        while cursor < program.len() {
            if let Some(by) = puter.apply_instruction(program[cursor]) {
                cursor = (cursor as isize + by) as usize;
            } else {
                cursor += 1;
            }
        }

        println!("value in b = {}", puter.b_val());
    }

    Ok(())
}
