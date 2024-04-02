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
    pub fn fetch_instruction(&mut self) -> Option<&Instruction> {
        self.instructions.get(self.pc)
    }
    pub fn inc_pc(&mut self) {
        self.pc += 1;
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
    /// Produces a ready-to-run program from src.
    /// In case of an error will return a String describing it.
    pub fn parse(src: &str) -> Result<Program, String> {
        Self::check_brackets(src).map_err(|e| e.to_string())?;

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

            let op = match c {
                '-' => Operation::CellDec,
                '+' => Operation::CellInc,
                '<' => Operation::TapeLeft,
                '>' => Operation::TapeRight,
                ',' => Operation::CellWrite,
                '.' => Operation::CellRead,
                '[' => Operation::BeginLoop(None),
                ']' => Operation::EndLoop,
                '!' => {
                    if i + 4 < chars.len() && chars[i + 1..i + 5] == ['T', 'A', 'P', 'E'] {
                        i += 4;
                        pos.char_number += 4;
                        Operation::TapePrint
                    } else {
                        pos.char_number += 1;
                        i += 1;
                        continue;
                    }
                }
                _ => {
                    pos.char_number += 1;
                    i += 1;
                    continue;
                }
            };

            // if a character is one of the groupable ones, find out how many times
            // it is repeating, so that we can then combine, the same operation done multiple times,
            // into single instruction
            let mut n = 1;
            if op == Operation::CellDec
                || op == Operation::CellInc
                || op == Operation::TapeLeft
                || op == Operation::TapeRight
            {
                let mut j = i + 1;
                while j < chars.len() && chars[j] == c {
                    n += 1;
                    j += 1;
                }
                program.instructions.push(Instruction::new(n, op, pos));
                pos.char_number += j - i;
                i = j;
            } else {
                program.instructions.push(Instruction::new(n, op, pos));
                pos.char_number += 1;
                i += 1
            }
        }

        // NOTE: this should probably not be a job of a parser but because of
        // how simple brainfuck is, this won't cost much and this way it won't
        // create any more steps going from src to runnable program
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

    /// The point of this function is to fill in all BeginLoop's addresses which store
    /// the position of their matching EndLoop, so that at runtime skipping a loop is trivial!
    /// This is a seperate function from `parse` to take away complexity from it having to
    /// also keep track of positions of all the loops and can just focus on producing
    /// an AST, which is only then fed into this function.
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

#[cfg(test)]
mod parser_tests {
    use super::*;
    #[test]
    fn test_operations() {
        assert_eq!(
            Parser::parse("-").unwrap().instructions[0].get_op(),
            &Operation::CellDec
        );
        assert_eq!(
            Parser::parse("+").unwrap().instructions[0].get_op(),
            &Operation::CellInc
        );
        assert_eq!(
            Parser::parse("<").unwrap().instructions[0].get_op(),
            &Operation::TapeLeft
        );
        assert_eq!(
            Parser::parse(">").unwrap().instructions[0].get_op(),
            &Operation::TapeRight
        );
        assert_eq!(
            Parser::parse(",").unwrap().instructions[0].get_op(),
            &Operation::CellWrite
        );
        assert_eq!(
            Parser::parse(".").unwrap().instructions[0].get_op(),
            &Operation::CellRead
        );

        let p = Parser::parse("[]").unwrap();
        assert!(matches!(
            p.instructions[0].get_op(),
            Operation::BeginLoop(_)
        ));
        assert_eq!(p.instructions[1].get_op(), &Operation::EndLoop);

        assert_eq!(
            Parser::parse("!TAPE").unwrap().instructions[0].get_op(),
            &Operation::TapePrint
        );
    }
    #[test]
    fn test_comments() {
        assert_eq!(Parser::parse("").unwrap().instructions.len(), 0);
        assert_eq!(Parser::parse("a").unwrap().instructions.len(), 0);
        assert_eq!(
            Parser::parse("!\nT\nA\nP\nE").unwrap().instructions.len(),
            0
        );
    }

    #[test]
    fn test_check_brackets() {
        assert_eq!(Parser::check_brackets("[]"), Ok(()));
        assert_eq!(Parser::check_brackets("[[]]"), Ok(()));
        assert_eq!(Parser::check_brackets("[[][]]"), Ok(()));
        assert_eq!(
            Parser::check_brackets("["),
            Err(BracketCountMismatch::MoreOpening(Position {
                line_number: 0,
                char_number: 0
            }))
        );
        assert_eq!(
            Parser::check_brackets("]"),
            Err(BracketCountMismatch::MoreClosing(Position {
                line_number: 0,
                char_number: 0
            }))
        );
        assert_eq!(
            Parser::check_brackets("...["),
            Err(BracketCountMismatch::MoreOpening(Position {
                line_number: 0,
                char_number: 3
            }))
        );
        assert_eq!(
            Parser::check_brackets("]["),
            Err(BracketCountMismatch::MoreClosing(Position {
                line_number: 0,
                char_number: 0
            }))
        );
    }

    #[test]
    fn test_instruction_grouping() {
        let p = Parser::parse("<>>---++++").unwrap();

        assert_eq!(p.instructions[0].get_n(), 1);
        assert_eq!(p.instructions[0].get_op(), &Operation::TapeLeft);

        assert_eq!(p.instructions[1].get_n(), 2);
        assert_eq!(p.instructions[1].get_op(), &Operation::TapeRight);

        assert_eq!(p.instructions[2].get_n(), 3);
        assert_eq!(p.instructions[2].get_op(), &Operation::CellDec);

        assert_eq!(p.instructions[3].get_n(), 4);
        assert_eq!(p.instructions[3].get_op(), &Operation::CellInc);
    }
    #[test]
    fn test_instruction_not_grouping() {
        let p = Parser::parse("[[,,..]]").unwrap();

        // check if first two brackets are seperate operations
        assert_eq!(p.instructions[0].get_n(), 1);
        assert!(matches!(
            p.instructions[0].get_op(),
            &Operation::BeginLoop(_)
        ));
        assert_eq!(p.instructions[1].get_n(), 1);
        assert!(matches!(
            p.instructions[1].get_op(),
            &Operation::BeginLoop(_)
        ));

        assert_eq!(p.instructions[2].get_n(), 1);
        assert_eq!(p.instructions[2].get_op(), &Operation::CellWrite);
        assert_eq!(p.instructions[3].get_n(), 1);
        assert_eq!(p.instructions[3].get_op(), &Operation::CellWrite);
        assert_eq!(p.instructions[4].get_n(), 1);
        assert_eq!(p.instructions[4].get_op(), &Operation::CellRead);
        assert_eq!(p.instructions[5].get_n(), 1);
        assert_eq!(p.instructions[5].get_op(), &Operation::CellRead);
    }
    #[test]
    fn test_loop_parsing() {
        let p = Parser::parse("[-]").unwrap();
        assert_eq!(p.instructions[0].get_n(), 1);
        assert_eq!(p.instructions[0].get_op(), &Operation::BeginLoop(Some(2)));

        assert_eq!(p.instructions[2].get_n(), 1);
        assert_eq!(p.instructions[2].get_op(), &Operation::EndLoop);
    }
    #[test]
    fn test_instruction_positioning() {
        assert_eq!(
            Parser::parse(" .").unwrap().instructions[0].get_position(),
            &Position {
                line_number: 0,
                char_number: 1
            }
        );
        assert_eq!(
            Parser::parse("\n.").unwrap().instructions[0].get_position(),
            &Position {
                line_number: 1,
                char_number: 0
            }
        );
        let p = Parser::parse("\n+-\n-+").unwrap();
        assert_eq!(
            p.instructions[0].get_position(),
            &Position {
                line_number: 1,
                char_number: 0
            }
        );
        assert_eq!(
            p.instructions[1].get_position(),
            &Position {
                line_number: 1,
                char_number: 1
            }
        );
        assert_eq!(
            p.instructions[2].get_position(),
            &Position {
                line_number: 2,
                char_number: 0
            }
        );
        assert_eq!(
            p.instructions[3].get_position(),
            &Position {
                line_number: 2,
                char_number: 1
            }
        );
    }
}
