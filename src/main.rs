mod tape;
use tape::Tape;

fn main() {
    let mut tape = Tape::new();

    println!("{}", tape);
    tape.move_right();
    tape.move_right();
    tape.move_right();
    tape.move_right();
    tape.move_right();
    tape.move_right();
    println!("{}", tape);
}
