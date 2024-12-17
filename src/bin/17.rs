#[allow(unused_imports)]
use advent_of_code::*;
use itertools::join;
use rayon::prelude::*;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    ChronospatialComputer::new(input).run_and_output()
}

pub fn part_two(input: &str) -> Option<usize> {
    let computer = ChronospatialComputer::new(input);
    let mut partial_match = (0, 0);
    loop {
        match (1..usize::MAX)
            .into_iter()
            .map(|a| (a << partial_match.1) | partial_match.0)
            .filter(|a| *a > 0b111) // If we find a partial match that's too small it fails
            .find_map(|a| {
                computer
                    .with_register_a(a)
                    .run_and_match_output(&computer.code)
                    .and_then(|full_match| Some((a, full_match)))
            }) {
            Some((n, true)) => {
                return Some(n);
            }
            Some((n, false)) => {
                partial_match = (n, numbits(n));
            }
            None => {
                return None;
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from(value: usize) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction {:?}", value),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct ChronospatialComputer {
    code: Vec<usize>,
    register_a: usize,
    register_b: usize,
    register_c: usize,
    ptr: usize,
}

impl ChronospatialComputer {
    fn new(input: &str) -> Self {
        let ((register_a, register_b, register_c), code) = parser!(
            section(
                line("Register A: " usize)
                line("Register B: " usize)
                line("Register C: " usize)
            )
            section(line("Program: " repeat_sep(usize, ",")))
        )
        .parse(input)
        .unwrap();

        Self {
            code,
            register_a,
            register_b,
            register_c,
            ptr: 0,
        }
    }

    fn with_register_a(&self, a: usize) -> Self {
        let mut computer = self.clone();
        computer.register_a = a;
        computer
    }

    fn run_and_output(&mut self) -> Option<String> {
        let mut output = vec![];
        if self._run(|v| {
            output.push(v);
            true
        }) {
            Some(join(&output, ","))
        } else {
            None
        }
    }

    fn run_and_match_output(&mut self, match_output: &[usize]) -> Option<bool> {
        let mut match_ptr = 0;
        self._run(|v| {
            if v == match_output[match_ptr] {
                match_ptr += 1;
                true // keep matching
            } else {
                false // stop matching
            }
        })
        .then(|| match_ptr == self.code.len())
    }

    fn _run<F>(&mut self, mut on_output: F) -> bool
    where
        F: FnMut(usize) -> bool,
    {
        while let Some((instruction, operand)) = self.read() {
            match instruction {
                Instruction::Adv => {
                    self.register_a = div_pow2(self.register_a, self.combo(operand));
                }
                Instruction::Bxl => {
                    self.register_b = self.register_b ^ self.literal(operand);
                }
                Instruction::Bst => {
                    self.register_b = self.combo(operand) % 8;
                }
                Instruction::Jnz => {
                    if self.register_a != 0 {
                        self.ptr = self.literal(operand);
                    }
                }
                Instruction::Bxc => {
                    // ignores operand
                    self.register_b = self.register_b ^ self.register_c;
                }
                Instruction::Out => {
                    if !on_output(self.combo(operand) % 8) {
                        return false;
                    }
                }
                Instruction::Bdv => {
                    self.register_b = div_pow2(self.register_a, self.combo(operand));
                }
                Instruction::Cdv => {
                    self.register_c = div_pow2(self.register_a, self.combo(operand));
                }
            }
        }
        true
    }

    fn literal(&self, operand: usize) -> usize {
        operand
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("Reserved - should not appear in program"),
            _ => panic!("Invalid operand for combo: {:?}", operand),
        }
    }

    fn read(&mut self) -> Option<(Instruction, usize)> {
        if self.ptr < self.code.len() {
            let instruction = Instruction::from(self.code[self.ptr]);
            let operand = self.code[self.ptr + 1];
            self.ptr += 2;
            Some((instruction, operand))
        } else {
            None
        }
    }
}

fn div_pow2(n: usize, pow: usize) -> usize {
    if pow == 0 {
        n
    } else {
        n / (2 << (pow - 1))
    }
}

fn numbits(mut v: usize) -> usize {
    if v == 0 {
        0
    } else {
        64 - v.leading_zeros() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut i = ChronospatialComputer {
            code: vec![2, 6],
            register_a: 1231,
            register_b: 823,
            register_c: 9,
            ptr: 0,
        };
        let _ = i.run_and_output();
        assert_eq!(i.register_b, 1);
    }

    #[test]
    fn test_b() {
        let mut i = ChronospatialComputer {
            code: vec![5, 0, 5, 1, 5, 4],
            register_a: 10,
            register_b: 823,
            register_c: 1829,
            ptr: 0,
        };
        let out = i.run_and_output();
        assert_eq!(out, Some("0,1,2".to_owned()));
    }

    #[test]
    fn test_c() {
        let mut i = ChronospatialComputer {
            code: vec![0, 1, 5, 4, 3, 0],
            register_a: 2024,
            register_b: 823,
            register_c: 1829,
            ptr: 0,
        };
        let out = i.run_and_output();
        assert_eq!(out, Some("4,2,5,6,7,7,7,7,3,1,0".to_owned()));
    }

    #[test]
    fn test_d() {
        let mut i = ChronospatialComputer {
            code: vec![1, 7],
            register_a: 2024,
            register_b: 29,
            register_c: 1829,
            ptr: 0,
        };
        let _ = i.run_and_output();
        assert_eq!(i.register_b, 26);
    }

    #[test]
    fn test_e() {
        let mut i = ChronospatialComputer {
            code: vec![4, 0],
            register_a: 2024,
            register_b: 2024,
            register_c: 43690,
            ptr: 0,
        };
        let _ = i.run_and_output();
        assert_eq!(i.register_b, 44354);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
