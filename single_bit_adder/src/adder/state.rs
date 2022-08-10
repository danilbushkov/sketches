
mod one;
pub mod zero;

//use crate::adder::state::State;


pub trait State {
    fn handler(&mut self, x: i8, y: i8) -> i8;
    fn get_next_state(&mut self) -> Box<dyn State>;
}



