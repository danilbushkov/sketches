
use crate::client::subscriber::Subscriber;

pub struct Subscriber1;

impl Subscriber for Subscriber1 {
    fn update(&mut self, event: &String) {
        println!("Subscriber1: {}", event);
    }
} 