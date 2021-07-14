use super::*;
use test_case::test_case;

use crate::unicodes::*;

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
fn test_line_nr(program: &str, program_counter: usize, expected_line_nr: usize) {
    assert_eq!(
        Traceback::line_number(program, program_counter),
        Ok(expected_line_nr)
    );
}

#[test_case("ABC\nDEFG\nHI", 0, 0, 0)]
#[test_case("ABC\nDEFG\nHI", 0, 2, 2)]
#[test_case("ABC\nDEFG\nHI", 1, 3, 0)]
#[test_case("ABC\nDEFG\nHI", 1, 6, 3)]
#[test_case("ABC\nDEFG\nHI", 2, 7, 0)]
#[test_case("ABC\nDEFG\nHI", 2, 8, 1)]
fn test_char_nr(program: &str, line_nr: usize, program_counter: usize, expected_char_nr: usize) {
    assert_eq!(
        Traceback::char_number(program, line_nr, program_counter),
        expected_char_nr
    );
}

#[test_case("😎BC\n🥳EFG\nHI", 0, 0)]
#[test_case("😎BC\n🥳EFG\nHI", 2, 0)]
#[test_case("😎BC\n🥳EFG\nHI", 3, 1)]
#[test_case("😎BC\n🥳EFG\nHI", 6, 1)]
#[test_case("😎BC\n🥳EFG\nHI", 7, 2)]
#[test_case("😎BC\n🥳EFG\nHI", 8, 2)]
fn test_line_nr_unicodes(program: &str, program_counter: usize, expected_line_nr: usize) {
    assert_eq!(
        Traceback::line_number(program, program_counter),
        Ok(expected_line_nr)
    );
}

#[test_case("😎BC\n🥳EFG\nHI", 0, 0, 0)]
#[test_case("😎BC\n🥳EFG\nHI", 0, 2, 2)]
#[test_case("😎BC\n🥳EFG\nHI", 1, 3, 0)]
#[test_case("😎BC\n🥳EFG\nHI", 1, 6, 3)]
#[test_case("😎BC\n🥳EFG\nHI", 2, 7, 0)]
#[test_case("😎BC\n🥳EFG\nHI", 2, 8, 1)]
fn test_char_nr_unicodes(
    program: &str,
    line_nr: usize,
    program_counter: usize,
    expected_char_nr: usize,
) {
    assert_eq!(
        Traceback::char_number(program, line_nr, program_counter),
        expected_char_nr
    );
}

#[test_case( 0, "a"   => format!("{}", "a".red())  ; "single character")]
#[test_case( 0, "𝚨"   => format!("{}", "𝚨" .red()) ; "unicode character")]
#[test_case( 0, "abc" => format!("{}bc", "a".red()) )]
#[test_case( 1, "abc" => format!("a{}c", "b".red()) )]
#[test_case( 2, "abc" => format!("ab{}", "c".red()) )]
#[test_case( 12, "pchnąć w tę łódź jeża lub ośm skrzyń fig"
    => format!("pchnąć w tę {}ódź jeża lub ośm skrzyń fig", "ł".red()) ; "long string with non ascii characters")]
fn test_highlighting(index: usize, test_text: &str) -> String {
    let test_text = &string_to_unicode_string(test_text);
    Traceback::highlight_current_char_in_line(test_text, index).unwrap()
}
