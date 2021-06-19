use super::*;

#[test]
fn test_current_line(){
    let (line_nr, char_nr, current_line) = Traceback::current_line("TEST 1\nTEST 2", 5);

    assert_eq!(line_nr, 2);
    assert_eq!(char_nr, 1);
    assert_eq!(current_line, "TEST 2");
}

#[test]
fn test_highlighting(){
    println!("{}", Traceback::highlight_current_char_in_line("Test 123", 3));
    assert_eq!(
        Traceback::highlight_current_char_in_line("Test 123", 3),
        format!("Tes{} 123", "t".red())
    );
}
