


use crate::client::subscriber::Subscriber;


pub struct Publisher<'a> {
    subscribers: Vec<&'a mut Box<dyn Subscriber>>,

}


impl<'a> Publisher<'a> {

    pub fn new() -> Self {
        Self{
            subscribers: vec![],
        }
    }


    pub fn subscribe(&mut self, subscriber: &'a mut Box<dyn Subscriber>) {
        self.subscribers.push(subscriber);
    }

    pub fn unsubscribe(&mut self, subscriber: &'a mut Box<dyn Subscriber>) {
        //
    }

    pub fn notify(&mut self, event: String) {
        for item in self.subscribers.iter_mut() {
            item.update(&event);
        }
    }
}