use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<Instruction>,
    pc: usize,
}

impl Program {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            pc: 0,
        }
    }
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
}

/// starts from 0
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub line_number: usize,
    pub char_number: usize,
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
    BeginLoop,
    EndLoop,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    /// number of times this operation should be repeated
    /// NOTE: BeginLoop and EndLoop will always have this set to 1
    n: u32,
    op: Operation,
    /// position of the first character of this operation in source code
    /// (line number, char number in line)
    pos: Position,
}

impl Instruction {
    fn new(n: u32, op: Operation, pos: Position) -> Self {
        Self { n, op, pos }
    }
    pub fn get_op(&self) -> &Operation {
        return &self.op;
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
        Parser::check_brackets(src).map_err(|e| e.to_string())?;

        let mut program = Program::new();
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
                '[' => Instruction::new(1, Operation::BeginLoop, pos),
                ']' => Instruction::new(1, Operation::EndLoop, pos),
                '!' => {
                    if i + 4 < chars.len() && chars[i + 1..i + 5] == ['T', 'A', 'P', 'E'] {
                        i += 4;
                        Instruction::new(1, Operation::TapePrint, pos)
                    } else {
                        continue;
                    }
                }
                _ => continue,
            });

            pos.char_number += 1;
        }

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
                } else if char == ']' {
                    if let None = opening_brackets.pop() {
                        return Err(BracketCountMismatch::MoreClosing(pos));
                    }
                }
            }
        }
        if let Some(pos) = opening_brackets.pop() {
            Err(BracketCountMismatch::MoreOpening(pos))
        } else {
            Ok(())
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
        })
    );
    assert_eq!(
        Parser::parse("+++\n+++"),
        Ok(Program {
            instructions: vec![Instruction::new(
                6,
                Operation::CellInc,
                Position {
                    line_number: 0,
                    char_number: 0
                }
            )],
            pc: 0,
        })
    );
}
