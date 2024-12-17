use aoc_traits::AdventOfCodeDay;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Computer {
    registers: [u64; 3],
    ip: usize,
    program: Vec<u8>,
}

impl Computer {
    fn combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.registers[A],
            5 => self.registers[B],
            6 => self.registers[C],
            _ => unreachable!(),
        }
    }

    pub fn run(&mut self) -> String {
        let mut output = String::new();
        loop {
            let instr = Instruction::from(self.program[self.ip]);
            let operand = self.program[self.ip + 1];

            match instr {
                Instruction::Adv => {
                    let num = self.registers[A];
                    let den = 2u64.pow(self.combo(operand) as u32);
                    self.registers[A] = num / den;
                    self.ip += 2;
                }
                Instruction::Bxl => {
                    self.registers[B] ^= operand as u64;
                    self.ip += 2;
                }
                Instruction::Bst => {
                    self.registers[B] = self.combo(operand) % 8;
                    self.ip += 2;
                }
                Instruction::Jnz => {
                    if self.registers[A] != 0 {
                        self.ip = operand as usize;
                    } else {
                        self.ip += 2;
                    }
                }
                Instruction::Bxc => {
                    self.registers[B] ^= self.registers[C];
                    self.ip += 2;
                }
                Instruction::Out => {
                    output += &(self.combo(operand) % 8).to_string();
                    output += ",";
                    self.ip += 2;
                }
                Instruction::Bdv => {
                    let num = self.registers[A];
                    let den = 2u64.pow(self.combo(operand) as u32);
                    self.registers[B] = num / den;
                    self.ip += 2;
                }
                Instruction::Cdv => {
                    let num = self.registers[A];
                    let den = 2u64.pow(self.combo(operand) as u32);
                    self.registers[C] = num / den;
                    self.ip += 2;
                }
            }

            if self.ip >= self.program.len() {
                output.remove(output.len() - 1);
                return output;
            }
        }
    }
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Computer;
    type Part1Output = String;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let (regs, program) = input.split_once("\n\n").unwrap();
        let regs = regs
            .lines()
            .map(|line| line["Register X: ".len()..].parse().unwrap())
            .collect::<Vec<_>>();
        let program = program["Program: ".len()..]
            .trim()
            .split(',')
            .map(|e| e.parse().unwrap())
            .collect();
        Computer {
            registers: [regs[0], regs[1], regs[2]],
            ip: 0,
            program,
        }
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut computer = input.to_owned();
        computer.run()
    }

    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;
    use aoc_traits::AdventOfCodeDay;

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    #[test]
    fn test_part1() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(
            Solver::solve_part1(&parsed),
            "4,6,3,5,6,3,5,2,1,0".to_owned()
        );
    }

    #[test]
    fn test_part2() {
        let parsed = Solver::parse_input(INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 31);
    }
}
