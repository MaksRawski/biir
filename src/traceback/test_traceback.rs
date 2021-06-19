use super::*;

#[test]
fn test_current_line_0(){
    let (line_nr, char_nr, current_line) = Traceback::current_line("TEST 1\nTEST 2", 0);

    assert_eq!(line_nr, 0);
    assert_eq!(char_nr, 0);
    assert_eq!(current_line, "TEST 1");
}

#[test]
fn test_current_line_1(){
    let (line_nr, char_nr, current_line) = Traceback::current_line("TEST 1\nTEST 2", 5);

    assert_eq!(line_nr, 0);
    assert_eq!(char_nr, 5);
    assert_eq!(current_line, "TEST 1");
}

#[test]
fn test_current_line_2(){
    let (line_nr, char_nr, current_line) = Traceback::current_line("TEST 1\nTEST 2", 6);

    assert_eq!(line_nr, 1);
    assert_eq!(char_nr, 0);
    assert_eq!(current_line, "TEST 2");
}

#[test]
fn test_current_line_3(){
    let (line_nr, char_nr, current_line) = Traceback::current_line("TEST 1\nTEST 2", 11);

    assert_eq!(line_nr, 1);
    assert_eq!(char_nr, 5);
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
