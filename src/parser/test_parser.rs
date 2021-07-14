use super::*;

use test_case::test_case;

#[test_case("["      => Err(()) ; "test 0")]
#[test_case("]"      => Err(()) ; "test 1")]
#[test_case("[]"     => Ok(1)   ; "test 2")]
#[test_case("[[[]]]" => Ok(5)   ; "test 3")]
#[test_case("[[][]]" => Ok(5)   ; "test 4")]
fn test_closing_bracket_finder(program: &str) -> Result<usize, ()> {
    let parser = Parser::new();
    let program = program.graphemes(true).collect::<Vec<&str>>();
    parser.find_closing_bracket(&program)
}
