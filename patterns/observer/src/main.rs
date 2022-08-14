mod client;


use crate::client::Client;
use crate::client::subscriber::Subscriber;
use crate::client::subscriber::subscriber1::Subscriber1;
use crate::client::subscriber::subscriber2::Subscriber2;

fn main() {

    
    let mut client = Client::new();

    let mut subscriber1: Box<dyn Subscriber> = Box::new(Subscriber1);
    let mut subscriber2: Box<dyn Subscriber> = Box::new(Subscriber2);

    client.subscribe(&mut subscriber1);
    client.open();
    client.subscribe(&mut subscriber2);

    client.close();

    
}
