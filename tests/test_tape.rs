extern crate biir;
use biir::tape::Tape;
use std::num::Wrapping;

#[cfg(test)]
mod test_tape {
    use super::*;

    #[test]
    fn test_moving() {
        let mut tape = Tape::new();
        // assert twice to make sure that it didn't actaully overflow
        assert_ne!(tape.move_left(1), Ok(()));
        assert_ne!(tape.move_left(1), Ok(()));

        for _ in 0..u16::MAX {
            assert_eq!(tape.move_right(1), Ok(()));
        }

        assert_ne!(tape.move_right(1), Ok(()));
        assert_ne!(tape.move_right(1), Ok(()));
    }

    #[test]
    fn test_current_value() {
        let mut tape = Tape::new();

        assert_eq!(tape.current_value, Wrapping(0));
        assert_ne!(tape.move_left(1), Ok(()));
        assert_eq!(tape.current_value, Wrapping(0));

        for _ in 0..u16::MAX {
            assert_eq!(tape.move_right(1), Ok(()));
            assert_eq!(tape.current_value, Wrapping(0));
        }

        assert_ne!(tape.move_right(1), Ok(()));
        assert_eq!(tape.current_value, Wrapping(0));
    }
}
