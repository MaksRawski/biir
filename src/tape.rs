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

impl From<Vec<usize>> for Tape {
    fn from(vec: Vec<usize>) -> Self {
        Self {
            tape: vec.iter().map(|v| Wrapping(*v)).collect(),
            ..Default::default()
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
        // highlight current cell and print upto 10 nearby cells
        // while trying to be in the middle
        let mut down_range = (self.current_position as isize - 5).max(0) as usize;
        let up_range = (self.current_position + 4)
            .max(down_range + 10)
            .min(self.tape.len());

        if up_range as isize - 10 > 0 {
            down_range = up_range - 10;
        }

        let mut res = String::new();

        if down_range > 0 {
            res = format!("({}) ... ", down_range);
        }

        for i in down_range..up_range {
            let value = self.tape.get(i);
            match value {
                Some(v) => {
                    if i == self.current_position {
                        res.push_str(&format!("[{}] ", v));
                    } else {
                        res.push_str(&format!("{} ", v));
                    }
                }
                None => break,
            }
        }

        res.pop();
        if up_range < self.tape.len() {
            res.push_str(&format!(" ... ({})", self.tape.len() - up_range));
        }
        write!(f, "{}", res)
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
    #[test]
    fn test_tape_display() {
        assert_eq!(Tape::from(vec![1, 2, 3]).to_string(), "[1] 2 3");
        assert_eq!(
            Tape::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).to_string(),
            "[1] 2 3 4 5 6 7 8 9 10"
        );

        let mut tape = Tape::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        tape.move_right(5).unwrap();
        assert_eq!(tape.to_string(), "1 2 3 4 5 [6] 7 8 9 10 ... (2)");

        tape.move_right(1).unwrap();
        assert_eq!(tape.to_string(), "(1) ... 2 3 4 5 6 [7] 8 9 10 11 ... (1)");

        tape.move_right(1).unwrap();
        assert_eq!(tape.to_string(), "(2) ... 3 4 5 6 7 [8] 9 10 11 12");

        tape.move_right(1).unwrap();
        assert_eq!(tape.to_string(), "(2) ... 3 4 5 6 7 8 [9] 10 11 12");

        tape.move_right(2).unwrap();
        assert_eq!(tape.to_string(), "(2) ... 3 4 5 6 7 8 9 10 [11] 12");

        tape.move_right(1).unwrap();
        assert_eq!(tape.to_string(), "(2) ... 3 4 5 6 7 8 9 10 11 [12]");

        let mut tape: Tape = (1..=20).collect::<Vec<usize>>().into();
        tape.move_right(9).unwrap();
        assert_eq!(
            tape.to_string(),
            "(4) ... 5 6 7 8 9 [10] 11 12 13 14 ... (6)"
        );
    }
}
