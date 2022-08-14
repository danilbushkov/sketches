mod publisher;
pub mod subscriber;

use crate::client::publisher::Publisher;
use crate::client::subscriber::Subscriber;


pub struct Client<'a> {
    publisher: Publisher<'a>,
}


impl<'a> Client<'a> {

    pub fn new() -> Self {
        Self{
            publisher: Publisher::new(),
        }
    }

    pub fn subscribe(&mut self, subscriber: &'a mut Box<dyn Subscriber>) {
        self.publisher.subscribe(subscriber);
    }

    pub fn open(&mut self) {
        self.publisher.notify("Open".to_string());
    }

    pub fn close(&mut self) {
        self.publisher.notify("Close".to_string());
    }
}