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

    pub fn set_current_value(&mut self, value: Wrapping<u8>){
        self.tape[ self.current_position as usize ] = value;
        self.current_value = value;
    }

    // TODO: we might not even want those functions to be public
    pub fn move_right(&mut self) -> Result<(), String>{
        // will return "Exceeded tape length" if current_position == 2**16
        // otherwise will return None
        if self.current_position == u16::MAX{
            return Err( String::from("Exceeded tape length") );
        }

        self.current_position += 1;
        match self.tape.get(self.current_position as usize){
            Some(v) => self.current_value = *v,
            None => {
                self.current_value = Wrapping(u8::MIN);
                self.tape.push(self.current_value);
            }
        }
        Ok( () )
    }

    pub fn move_left(&mut self) -> Result<(), String>{
        if self.current_position == u16::MIN{
            return Err( String::from("Tried to go to the negative side of the tape") );
        }

        self.current_position -= 1;
        self.current_value = *self.tape.get(self.current_position as usize).unwrap();
        Ok( () )
    }

    pub fn inc(&mut self){
        self.set_current_value( self.current_value + Wrapping(1) );
    }

    pub fn dec(&mut self){
        self.set_current_value( self.current_value - Wrapping(1) );
    }
}

impl fmt::Display for Tape{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // highlight current cell and print nearby cells
        // print 10 cells while trying to be in the middle
        let down_range = (self.current_position as i16 - 5).max(0) as u16;
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
        if down_range > u16::MIN{
            tape = format!("current postion: {}\n... {}", self.current_position, tape);
        }
        // if we are on the last created cell
        // we don't want to print "..." as if there is something further
        if self.current_position as usize != self.tape.len() - 1 && up_range < u16::MAX{
            tape = format!("{}...", tape);
        }
        write!(f, "{}", tape)
    }
}

