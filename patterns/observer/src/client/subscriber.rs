pub mod subscriber1;
pub mod subscriber2;


pub trait Subscriber {
    fn update(&mut self, event: &String);

}