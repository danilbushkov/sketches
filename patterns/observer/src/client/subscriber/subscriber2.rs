
use crate::client::subscriber::Subscriber;

pub struct Subscriber2;




impl Subscriber for Subscriber2 {
    fn update(&mut self, event: &String) {
        println!("Subscriber2: {}", event);
    }
} 

