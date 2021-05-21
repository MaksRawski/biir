use std::fmt;
use std::num::Wrapping;

pub struct Tape {
    pub current_position: u16,
    pub current_value: Wrapping<u8>,
    tape: Box<Vec<Wrapping<u8>>>,
}

impl Tape {
    pub fn new() -> Tape {
        Tape {
            current_position: 0,
            current_value: Wrapping(0),
            tape: Box::new(Vec::from( [Wrapping(0)] )),
        }
    }

    // TODO: we might not even want those functions to be public
    pub fn move_right(&mut self) -> Option<String>{
        // will return "Exceeded tape length" if current_position == 2**16
        // otherwise will return None
        if self.current_position == u16::MAX{
            return Some(String::from("Exceeded tape length"));
        }

        self.current_position += 1;
        match self.tape.get(self.current_position as usize){
            Some(v) => self.current_value = *v,
            None => {
                self.current_value = Wrapping(u8::MIN);
                self.tape.push(self.current_value);
            }
        }
        None
    }

    pub fn move_left(&mut self) -> Option<String>{
        if self.current_position == u16::MIN{
            return Some(String::from("Tried to go to the negative side of the tape"));
        }

        self.current_position -= 1;
        self.current_value = *self.tape.get(self.current_position as usize).unwrap();
        None
    }

    pub fn inc(&mut self){
        self.current_value += Wrapping(1);
        self.tape[self.current_position as usize] = self.current_value;
    }

    pub fn dec(&mut self){
        self.current_value -= Wrapping(1);
        self.tape[self.current_position as usize] = self.current_value;
    }
}

impl fmt::Display for Tape{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // highlight current cell and print nearby cells
        // print 10 cells while trying to be in the middle
        let down_range = (self.current_position as i16 - 5).max(0);
        let up_range = down_range + 10;
        let mut tape = String::new();

        for i in down_range..up_range{
            let value = self.tape.get(i as usize);
            match value{
                Some(v) => {
                    if i as u16 == self.current_position{
                        tape = format!("{}[{}] ", tape, v);
                    }
                    else{
                        tape = format!("{}{} ", tape, v);
                    }
                }
                None => break,
            }
        }
        write!(f, "{}", tape)
    }
}

