use super::utils::{Address, Position};

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
    pub fn new(n: usize, op: Operation, pos: Position) -> Self {
        Self { n, op, pos }
    }
    pub fn get_op(&self) -> &Operation {
        &self.op
    }
    pub fn get_n(&self) -> usize {
        self.n
    }
    pub fn get_position(&self) -> &Position {
        &self.pos
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
