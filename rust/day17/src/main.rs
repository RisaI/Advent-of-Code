use anyhow::Context;
use winnow::{
    ascii::{alpha1, digit1, line_ending},
    combinator::{preceded, separated_pair},
    error::InputError,
    Parser,
};

enum OperationOutput {
    Out(u8),
    Jump(usize),
    None,
}

struct Puter {
    pub registers: [usize; 3],
}

impl Puter {
    pub fn new_a(a: usize) -> Self {
        Self {
            registers: [a, 0, 0],
        }
    }

    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let registers: Vec<_> = winnow::combinator::separated(
            3..=3,
            separated_pair(
                preceded("Register ", alpha1::<_, InputError<_>>),
                ": ",
                digit1.parse_to::<usize>(),
            ),
            line_ending,
        )
        .parse(input)
        .map_err(|e| anyhow::format_err!("{e}"))?;

        if !(('A'..='C').eq(registers.iter().filter_map(|(v, _)| v.chars().next()))) {
            anyhow::bail!("register names are off");
        }

        Ok(Self {
            registers: std::array::from_fn(|i| registers[i].1),
        })
    }

    pub fn resolve_operand(&self, val: u8, combo: bool) -> usize {
        if val > 7 {
            panic!("invalid operand");
        }

        if !combo {
            return val as usize;
        }

        match val {
            0..=3 => val as usize,
            4..=6 => self.registers[val as usize - 4],
            7 => panic!("reserved instruction"),
            _ => unreachable!(),
        }
    }

    pub fn perform_operation(&mut self, opcode: u8, operand: u8) -> OperationOutput {
        if opcode > 7 {
            panic!("invalid opcode");
        }

        match opcode {
            0 => {
                // adv
                self.registers[0] >>= self.resolve_operand(operand, true);
            }
            1 => {
                // bxl
                self.registers[1] ^= self.resolve_operand(operand, false);
            }
            2 => {
                // bst
                self.registers[1] = self.resolve_operand(operand, true) % 8;
            }
            3 => {
                // jnz
                if self.registers[0] != 0 {
                    return OperationOutput::Jump(self.resolve_operand(operand, false));
                }
            }
            4 => {
                // bxc
                self.registers[1] ^= self.registers[2];
            }
            5 => {
                // out
                return OperationOutput::Out((self.resolve_operand(operand, true) % 8) as u8);
            }
            6 => {
                // bdv
                self.registers[1] = self.registers[0] >> self.resolve_operand(operand, true);
            }
            7 => {
                // cdv
                self.registers[2] = self.registers[0] >> self.resolve_operand(operand, true);
            }
            _ => unreachable!(),
        }

        OperationOutput::None
    }

    pub fn eval(&mut self, program: &[u8], expected_output: Option<&[u8]>) -> Option<Vec<u8>> {
        let mut cursor = 0;
        let mut output = vec![];

        while cursor < program.len() - 1 {
            match self.perform_operation(program[cursor], program[cursor + 1]) {
                OperationOutput::None => (),
                OperationOutput::Out(p) => {
                    if let Some(expected_output) = expected_output {
                        if output.len() == expected_output.len()
                            || expected_output[output.len()] != p
                        {
                            return None;
                        }
                    }

                    output.push(p);
                }
                OperationOutput::Jump(v) => {
                    cursor = v;
                    continue;
                }
            }

            cursor += 2;
        }

        Some(output)
    }
}

fn find_initial_register(program: &[u8]) -> Option<usize> {
    // This algorithm uses external knowledge:
    // * the program is advanced by right-shifting A by 3 bits
    // * other registers are dependent solely on A

    fn inner(program: &[u8], candidate: usize, idx: usize) -> Option<usize> {
        for i in 0..8 {
            let test = (candidate << 3) | i;

            let mut puter = Puter::new_a(test);

            if puter.eval(program, Some(&program[idx..])).is_some() {
                if idx == 0 {
                    return Some(test);
                }

                if let Some(val) = inner(program, test, idx - 1) {
                    return Some(val);
                }
            }
        }

        None
    }

    inner(program, 0, program.len() - 1)
}

#[test]
fn example_werks() {
    let mut puter = Puter::new_a(729);

    assert_eq!(
        puter.eval(&[0, 1, 5, 4, 3, 0], None).unwrap(),
        vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0],
    )
}

fn solution_to_string(solution: &[u8]) -> String {
    solution
        .iter()
        .enumerate()
        .fold(String::new(), |mut s, (i, v)| {
            if i > 0 {
                s.push(',');
            }

            s.push_str(&v.to_string());

            s
        })
}

fn main() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("data.txt")?;
    let (puter, program) = data
        .split_once("\n\n")
        .context("input is in a wrong format")?;
    let (_, program) = program
        .split_once(' ')
        .context("program is in a wrong format")?;

    let mut puter = Puter::parse(puter)?;
    let program = program
        .trim()
        .split(',')
        .map(|v| v.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()?;

    // P1
    {
        let output = puter.eval(&program, None).unwrap();

        println!("{}", solution_to_string(&output));
    }

    // P2
    let result = find_initial_register(&program).context("solution for p2 not found")?;
    println!("A value to replicate program: {result}");

    Ok(())
}
