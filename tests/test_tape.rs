extern crate biir;
use biir::tape::Tape;
use std::num::Wrapping;

#[cfg(test)]
mod test_tape{
    use super::*;

    #[test]
    #[ignore]
    fn test_invalid_position(){
        // set_current_value can panic but it won't if ve use move_* functions
        let mut tape = Tape::new();
        tape.current_position = 5;
        tape.set_current_value(Wrapping(255));
    }

    #[test]
    fn test_moving(){
        let mut tape = Tape::new();
        // assert twice to make sure that it didn't actaully overflow
        assert_ne!( tape.move_left(), Ok( () ));
        assert_ne!( tape.move_left(), Ok( () ));

        for _ in 0..u16::MAX{
            assert_eq!( tape.move_right(), Ok( () ));
        }

        assert_ne!( tape.move_right(), Ok( () ));
        assert_ne!( tape.move_right(), Ok( () ));
    }

    #[test]
    fn test_current_value(){
        let mut tape = Tape::new();

        assert_eq!( tape.current_value, Wrapping(0) );
        assert_ne!( tape.move_left(), Ok( () ));
        assert_eq!( tape.current_value, Wrapping(0) );

        for _ in 0..u16::MAX{
            assert_eq!( tape.move_right(), Ok( () ));
            assert_eq!( tape.current_value, Wrapping(0) );
        }

        assert_ne!( tape.move_right(), Ok( () ) );
        assert_eq!( tape.current_value, Wrapping(0) );
    }
}
