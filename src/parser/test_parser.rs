use super::*;

#[test]
fn test_closing_bracket_finder(){
    let parser = Parser::new();
    assert_eq!( parser.find_closing_bracket("["), Err(()) );
    assert_eq!( parser.find_closing_bracket("]"), Err(()) );
    assert_eq!( parser.find_closing_bracket("[]"), Ok(1) );
    assert_eq!( parser.find_closing_bracket("[[[]]]"), Ok(5) );
    assert_eq!( parser.find_closing_bracket("[[][]]"), Ok(5) );
}
