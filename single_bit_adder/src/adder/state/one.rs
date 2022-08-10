

use crate::adder::state::State;
use crate::adder::state::zero::Zero;

pub struct One {
    next_state: Option<Box<dyn State>>,
}


impl One {
    pub fn new() -> Self {
        Self {
            next_state: None,
        }
    }
}




impl State for One {
    fn handler(&mut self, x: i8, y: i8) -> i8 {

        let mut sum = x + y;
        if sum == 0 {
            sum = 1;
            self.next_state = Some(Box::new(Zero::new()));
        } else {
            self.next_state = Some(Box::new(One::new()));
            if sum == 2 {
                sum = 1;
            } else {
                sum = 0;
            }
        }

        sum
    }
    fn get_next_state(&mut self) -> Box<dyn State> {
        self.next_state.take().unwrap()
    }

}