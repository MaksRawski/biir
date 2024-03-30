use std::fmt::Display;
type Address = usize;

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

/// Handy struct for storing position in file.
/// Both line_number and char_number should start from 0.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub line_number: usize,
    pub char_number: usize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.line_number == other.line_number {
            Some(self.char_number.cmp(&other.char_number))
        } else {
            Some(self.line_number.cmp(&other.line_number))
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "line {}, char {}",
            self.line_number + 1,
            self.char_number + 1
        ))
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    TapeLeft,
    TapeRight,
    /// Prints 10 nearby tape values.
    TapePrint,
    CellInc,
    CellDec,
    CellRead,
    CellWrite,
    /// Contains the address of the matching EndLoop or None if it wasn't set yet.
    BeginLoop(Option<Address>),
    EndLoop,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    /// number of times this operation should be repeated
    /// NOTE: BeginLoop and EndLoop will always have this set to 1
    n: usize,
    /// type of operation that will be performed
    op: Operation,
    /// position of the first appearance of this operation in source code
    /// (line number, char number in line)
    pos: Position,
}

impl Instruction {
    fn new(n: usize, op: Operation, pos: Position) -> Self {
        Self { n, op, pos }
    }
    pub fn get_op(&self) -> &Operation {
        &self.op
    }
    pub fn get_n(&self) -> usize {
        self.n
    }
    pub fn set_end_of_loop_address(&mut self, addr: Address) {
        match self.op {
            Operation::BeginLoop(None) => {
                self.op = Operation::BeginLoop(Some(addr));
            }
            Operation::BeginLoop(_) => {
                panic!("set_end_of_loop_address was called more than once!")
            }
            _ => {
                panic!(
                    "set_end_of_loop_address called on a non-BeginLoop instruction! instruction: {:?}",
                    self
                )
            }
        }
    }
}

pub enum BracketCountMismatch {
    MoreOpening(Position),
    MoreClosing(Position),
}

impl Display for BracketCountMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (bracket, pos) = if let BracketCountMismatch::MoreOpening(pos) = self {
            ("opening", pos)
        } else if let BracketCountMismatch::MoreClosing(pos) = self {
            ("closing", pos)
        } else {
            unreachable!();
        };
        f.write_fmt(format_args!(
            "Bracket count mismatch! Extra {} bracket found at {}",
            bracket, pos
        ))
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
