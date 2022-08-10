

use crate::adder::state::State;
use crate::adder::state::one::One;

pub struct Zero {
    next_state: Option<Box<dyn State>>,
}

impl Zero {
    pub fn new() -> Self {
        Self {
            next_state: None,
        }
    }
}

impl State for Zero {
    fn handler(&mut self, x: i8, y: i8) -> i8 {
        

        let mut sum = x + y;
        if sum == 2 {
            self.next_state = Some(Box::new(One::new()));
            sum = 0;
        } else {
            self.next_state = Some(Box::new(Zero::new()));
        }

        sum
    }
    fn get_next_state(&mut self) -> Box<dyn State> {
        self.next_state.take().unwrap()
    }

}