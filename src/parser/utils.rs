use core::fmt::Display;

pub type Address = usize;

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
