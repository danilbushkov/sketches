mod state;

use crate::adder::state::State;
use crate::adder::state::zero::Zero;


pub struct Adder {
    state: Box<dyn State>,
} 


impl Adder {

    pub fn new() -> Self {
        Self {
            state: Box::new(Zero::new()),
        }
    }


    pub fn add(&mut self, x: i8, y: i8) -> i8 {
        let result = self.state.handler(x, y);
        self.state = self.state.get_next_state();
        result 

    }
}