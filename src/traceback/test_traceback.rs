use super::*;
use test_case::test_case;
use proptest::prelude::*;

#[test_case(0,   0, 0, "TEST 1"; "first char of the first line")]
#[test_case(5,   0, 5, "TEST 1"; "last char of the first line")]
#[test_case(6,   1, 0, "TEST 2"; "first char of the second line")]
#[test_case(11,  1, 5, "TEST 2"; "last char of the second line")]
fn test_current_line(
    pc: usize,
    expected_line_nr: usize,
    expected_char_nr: usize,
    expected_line: &str,
) {
    let (line_nr, char_nr, current_line) = Traceback::current_line("TEST 1\nTEST 2", pc);

    assert_eq!(line_nr, expected_line_nr);
    assert_eq!(char_nr, expected_char_nr);
    assert_eq!(current_line, expected_line);
}

proptest!{
    #[test]
    fn test_highlighting(index: usize, test_text: String) {
        let current_char_red = test_text.chars().nth(index).unwrap().to_string().red();

        let output = Traceback::highlight_current_char_in_line(&test_text, index);

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
