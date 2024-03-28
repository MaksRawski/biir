use std::fmt;
use std::num::Wrapping;

use crate::error::Error;

pub struct Tape {
    pub current_position: usize,
    pub current_value: Wrapping<usize>,
    tape: Box<Vec<Wrapping<usize>>>,
}

impl Tape {
    pub fn new() -> Tape {
        Tape {
            current_position: 0,
            current_value: Wrapping(0),
            tape: Box::new(Vec::from([Wrapping(0)])),
        }
    }

    pub fn set_current_value(&mut self, value: Wrapping<usize>) {
        self.tape[self.current_position] = value;
        self.current_value = value;
    }

    pub fn move_right(&mut self) -> Result<(), Error> {
        if self.current_position == usize::MAX {
            return Err(Error::Runtime("Exceeded tape length".to_string()));
        }

        self.current_position += 1;
        match self.tape.get(self.current_position) {
            Some(v) => self.current_value = *v,
            None => {
                self.current_value = Wrapping(usize::MIN);
                self.tape.push(self.current_value);
            }
        }
        Ok(())
    }

    pub fn move_left(&mut self) -> Result<(), Error> {
        if self.current_position == usize::MIN {
            return Err(Error::Runtime(
                "Tried to go to the negative side of the tape".to_string(),
            ));
        }

        self.current_position -= 1;
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
            let value = self.tape.get(i as usize);
            match value {
                Some(v) => {
                    if i as usize == self.current_position {
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
        if self.current_position as usize != self.tape.len() - 1 && up_range < usize::MAX {
            tape = format!("{}...", tape);
        }
        write!(f, "{}", tape)
    }
}
