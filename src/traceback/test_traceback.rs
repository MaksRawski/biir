use super::*;
use test_case::test_case;
use proptest::prelude::*;

// Unfortunetly we can't just put those test_cases
// above definitions as they're inside impl block.
// Also moving it all into module instead would require
// test_case to be a custom test framework
// and would probably have to be included in normal dependencies

#[test_case("ABC\nDEFG\nHI", 0, 0)]
#[test_case("ABC\nDEFG\nHI", 2, 0)]
#[test_case("ABC\nDEFG\nHI", 3, 1)]
#[test_case("ABC\nDEFG\nHI", 6, 1)]
#[test_case("ABC\nDEFG\nHI", 7, 2)]
#[test_case("ABC\nDEFG\nHI", 8, 2)]
fn test_line_nr(program: &str, program_counter: usize, expected_line_nr: usize){
    assert_eq!(Traceback::line_number(program, program_counter), Ok( expected_line_nr) );
}

#[test_case("ABC\nDEFG\nHI", 0, 0, 0)]
#[test_case("ABC\nDEFG\nHI", 0, 2, 2)]
#[test_case("ABC\nDEFG\nHI", 1, 3, 0)]
#[test_case("ABC\nDEFG\nHI", 1, 6, 3)]
#[test_case("ABC\nDEFG\nHI", 2, 7, 0)]
#[test_case("ABC\nDEFG\nHI", 2, 8, 1)]
fn test_char_nr(program: &str, line_nr: usize, program_counter: usize, expected_char_nr: usize){
    assert_eq!(Traceback::char_number(program, line_nr, program_counter), expected_char_nr);
}

proptest!{
    #[test]
    fn test_highlighting(index: usize, test_text: String) {
        // skip empty strings as they're supposed to return Err
        if test_text.len() == 0 { return Ok( () ) }

        let output = Traceback::highlight_current_char_in_line(&test_text, index).unwrap();

        let current_char_red = test_text.chars().nth(index).unwrap();

        // each colored char takes 10 chars
        // if we collect to String it will be stored as 10 chars
        let letter_supposed_to_be_red = output.chars().skip(index).take(10).collect::<String>();

        assert_eq!(letter_supposed_to_be_red, current_char_red.to_string());

        // assert that there is only one occurence of colored char
        // to do that we check if final string's length
        // is just 9 chars longer than the test_text
        assert_eq!(output.len(), test_text.len() + 9);

        // TODO: assert that there is only single letter red
    }
}
