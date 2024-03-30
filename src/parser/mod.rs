pub mod instruction;
pub mod utils;
use crate::parser::instruction::{Instruction, Operation};
use utils::Position;

use self::utils::{Address, BracketCountMismatch};

#[derive(Debug, PartialEq, Default)]
pub struct Program {
    instructions: Vec<Instruction>,
    stack: Vec<usize>,
    pc: Address,
}

impl Program {
    pub fn reset(&mut self) {
        self.pc = 0;
    }
    pub fn next_instruction(&mut self) -> Option<&Instruction> {
        let i = self.instructions.get(self.pc);
        self.pc += 1;
        i
    }
    pub fn jump(&mut self, addr: usize) {
        if addr < self.instructions.len() {
            self.pc = addr;
        }
    }
    pub fn begin_loop(&mut self, cell_value: usize) {
        if cell_value == 0 {
            // skip the loop
            match self.instructions.get(self.pc) {
                Some(i) => match i.get_op() {
                    Operation::BeginLoop(possible_address) => match possible_address{
                        Some(jump_address) => self.jump(*jump_address),
                        None => panic!("begin_loop tried to skip loop but no matching EndLoop address was found! instruction: {:?}", i)
                    },
                    _ => panic!("begin_loop called on non-BeginLoop instruction: {:?}", i),
                },
                None => panic!("begin_loop called while outside the program address space! address: {}, program length: {}", self.pc, self.instructions.len()),
            };
        } else {
            // enter the loop and save the position on the stack
            self.stack.push(self.pc);
        }
    }
    pub fn end_loop(&mut self, cell_value: usize) {
        if cell_value == 0 {
            // exit the loop
            self.stack.pop();
        } else {
            // jump to start
            match self.stack.last() {
                Some(loop_start_address) => self.jump(*loop_start_address),
                None => panic!("Tried to end_loop when stack was empty!"),
            }
        }
    }
}

pub struct Parser;
impl Parser {
    pub fn parse(src: &str) -> Result<Program, String> {
        let mut program = Program::default();
        let chars = src.chars().collect::<Vec<_>>();
        let mut pos = Position {
            line_number: 0,
            char_number: 0,
        };

        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            // vim just prints '\r', emacs considers them newlines,
            // on windows '\r\n' is usually used at the end of lines,
            // so let's consider standalone '\r' as EOL and skip '\n' that follows
            if c == '\r' {
                if i + 1 < chars.len() && chars[i + 1] == '\n' {
                    i += 1;
                }
                pos.line_number += 1;
                pos.char_number = 0;
                i += 1;
                continue;
            } else if c == '\n' {
                pos.line_number += 1;
                pos.char_number = 0;
                i += 1;
                continue;
            }

            // find out how many times a character is repeating, so that we can then
            // combine, the same operation done multiple times, into single instruction
            let mut n = 1;
            let mut j = i + 1;
            while j < chars.len() && chars[j] == c {
                n += 1;
                j += 1;
            }
            i = j;

            program.instructions.push(match c {
                '-' => Instruction::new(n, Operation::CellDec, pos),
                '+' => Instruction::new(n, Operation::CellInc, pos),
                '<' => Instruction::new(n, Operation::TapeLeft, pos),
                '>' => Instruction::new(n, Operation::TapeRight, pos),
                // it doesn't really make sense to combine these operations below
                ',' => Instruction::new(1, Operation::CellWrite, pos),
                '.' => Instruction::new(1, Operation::CellRead, pos),
                '[' => Instruction::new(1, Operation::BeginLoop(None), pos),
                ']' => Instruction::new(1, Operation::EndLoop, pos),
                '!' => {
                    if i + 4 < chars.len() && chars[i + 1..i + 5] == ['T', 'A', 'P', 'E'] {
                        i += 4;
                        Instruction::new(1, Operation::TapePrint, pos)
                    } else {
                        continue;
                    }
                }
                _ => {
                    continue;
                }
            });

            pos.char_number += 1;
        }
        Self::fill_loops_addresses(&mut program.instructions);

        Ok(program)
    }

    pub fn check_brackets(src: &str) -> Result<(), BracketCountMismatch> {
        let mut opening_brackets: Vec<Position> = Vec::new();
        for (i, line) in src.lines().enumerate() {
            for (j, char) in line.chars().enumerate() {
                let pos = Position {
                    line_number: i,
                    char_number: j,
                };
                if char == '[' {
                    opening_brackets.push(pos);
                } else if char == ']' && opening_brackets.pop().is_none() {
                    return Err(BracketCountMismatch::MoreClosing(pos));
                }
            }
        }
        if let Some(pos) = opening_brackets.pop() {
            Err(BracketCountMismatch::MoreOpening(pos))
        } else {
            Ok(())
        }
    }

    fn fill_loops_addresses(instructions: &mut [Instruction]) {
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..instructions.len() {
            match instructions[i].get_op() {
                Operation::BeginLoop(None) => stack.push(i),
                Operation::EndLoop => match stack.pop() {
                    Some(beg) => match instructions.get_mut(beg) {
                        Some(begin_loop_instruction) => {
                            begin_loop_instruction.set_end_of_loop_address(i)
                        }
                        None => todo!("???"),
                    },
                    None => {
                        panic!("check_brackets failed to detect a mismatch in bracket count?")
                    }
                },
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod test_program_struct {
    #[test]
    fn test_program_begin_loop() {}
}

#[test]
fn test_parser() {
    assert_eq!(
        Parser::parse("---"),
        Ok(Program {
            instructions: vec![Instruction::new(
                3,
                Operation::CellDec,
                Position {
                    line_number: 0,
                    char_number: 0
                }
            )],
            pc: 0,
            stack: Vec::new(),
        })
    );
    // assert_eq!(
    //     Parser::parse("+++\n+++"),
    //     Ok(Program {
    //         instructions: vec![Instruction::new(
    //             6,
    //             Operation::CellInc,
    //             Position {
    //                 line_number: 0,
    //                 char_number: 0
    //             }
    //         )],
    //         pc: 0,
    //         stack: Vec::new(),
    //     })
    // );
}
