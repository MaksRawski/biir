use std::fmt;
use std::num::Wrapping;

// TODO: create a BigIntTape and CharTape, both of which somehow share this code
// by having this as implementation of a trait Tape

pub struct Tape {
    pub current_position: usize,
    pub current_value: Wrapping<usize>,
    tape: Vec<Wrapping<usize>>,
}

impl Default for Tape {
    fn default() -> Self {
        Self {
            current_position: 0,
            current_value: Wrapping(0),
            tape: vec![Wrapping(0)],
        }
    }
}

impl Tape {
    pub fn set_current_value(&mut self, value: Wrapping<usize>) {
        self.tape[self.current_position] = value;
        self.current_value = value;
    }

    pub fn move_right(&mut self, n: usize) -> Result<(), String> {
        if self.current_position == usize::MAX {
            return Err("Exceeded tape length".to_string());
        }

        self.current_position += n;
        match self.tape.get(self.current_position) {
            Some(v) => self.current_value = *v,
            None => {
                self.current_value = Wrapping(usize::MIN);
                self.tape.push(self.current_value);
            }
        }
        Ok(())
    }

    pub fn move_left(&mut self, n: usize) -> Result<(), String> {
        if self.current_position == usize::MIN {
            return Err("Tried to go to the negative side of the tape".to_string());
        }

        self.current_position -= n;
        self.current_value = *self.tape.get(self.current_position).unwrap();
        Ok(())
    }

    pub fn inc(&mut self, n: usize) {
        self.set_current_value(self.current_value + Wrapping(n));
    }

    pub fn dec(&mut self, n: usize) {
        self.set_current_value(self.current_value - Wrapping(n));
    }
}

impl fmt::Display for Tape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // highlight current cell and print nearby cells
        // print 10 cells while trying to be in the middle
        let down_range = (self.current_position as isize - 5).max(0) as usize;
        let up_range = down_range + 10;
        let mut tape = String::new();

        for i in down_range..up_range {
            let value = self.tape.get(i);
            match value {
                Some(v) => {
                    if i == self.current_position {
                        tape = format!("{}[{}] ", tape, v);
                    } else {
                        tape = format!("{}{} ", tape, v);
                    }
                }
                None => break,
            }
        }
        if down_range > usize::MIN {
            tape = format!("current postion: {}\n... {}", self.current_position, tape);
        }
        // if we are on the last created cell
        // we don't want to print "..." as if there is something further
        if self.current_position != self.tape.len() - 1 && up_range < usize::MAX {
            tape = format!("{}...", tape);
        }
        write!(f, "{}", tape)
    }
}

#[cfg(test)]
mod test_tape {
    use super::*;

    #[test]
    fn test_moving() {
        let mut tape = Tape::default();
        // assert twice to make sure that it didn't actaully overflow
        assert_ne!(tape.move_left(1), Ok(()));
        assert_ne!(tape.move_left(1), Ok(()));

        assert_eq!(tape.move_right(usize::MAX), Ok(()));
        assert_ne!(tape.move_right(1), Ok(()));
    }

    #[test]
    fn test_current_value() {
        let mut tape = Tape::default();

        assert_eq!(tape.current_value, Wrapping(0));
        assert_ne!(tape.move_left(1), Ok(()));
        assert_eq!(tape.current_value, Wrapping(0));

        assert_eq!(tape.move_right(usize::MAX), Ok(()));
        assert_eq!(tape.current_value, Wrapping(0));

        assert_ne!(tape.move_right(1), Ok(()));
        assert_eq!(tape.current_value, Wrapping(0));
    }

    #[test]
    fn test_value_changing() {
        let mut tape = Tape::default();
        tape.inc(usize::MAX);
        assert_eq!(tape.current_value, Wrapping(usize::MAX));
        tape.inc(1);
        assert_eq!(tape.current_value, Wrapping(0));
        tape.dec(1);
        assert_eq!(tape.current_value, Wrapping(usize::MAX));
    }
}
