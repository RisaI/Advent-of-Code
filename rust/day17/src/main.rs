enum OperationOutput {
    Out(u8),
    Jump(usize),
    None,
}

struct Puter {
    pub registers: [usize; 3],
}

impl Puter {
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
                println!("{}", self.registers[2]);
            }
            _ => unreachable!(),
        }

        OperationOutput::None
    }

    pub fn eval(&mut self, program: &[u8], expected_output: Option<&[u8]>) -> Vec<u8> {
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
                            break;
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

        output
    }
}

#[test]
fn example_werks() {
    let mut puter = Puter {
        registers: [729, 0, 0],
    };

    assert_eq!(
        puter.eval(&[0, 1, 5, 4, 3, 0], None),
        vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0],
    )
}

fn main() {
    let mut puter = Puter {
        registers: [37293246, 0, 0],
    };
    let program: Vec<u8> = vec![2, 4, 1, 6, 7, 5, 4, 4, 1, 7, 0, 3, 5, 5, 3, 0];
    let output = puter.eval(&program, None);

    println!(
        "{}",
        output
            .into_iter()
            .enumerate()
            .fold(String::new(), |mut s, (i, v)| {
                if i > 0 {
                    s.push(',');
                }

                s.push_str(&v.to_string());

                s
            })
    );

    let bound = 8usize.pow(program.len() as u32 - 1);

    for i in bound.. {
        let mut puter = Puter {
            registers: [i, 0, 0],
        };

        if puter.eval(&program, Some(&program)) == program {
            println!("Program replicated for A = {i}");
            break;
        }
    }
}
