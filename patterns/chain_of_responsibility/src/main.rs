fn main() {
    let mut handler1 = Box::new(ConcreteHandler1 {
        next: None,
    });

    let mut handler2 = Box::new(ConcreteHandler2 {
        next: None,
    });

    let handler3 = Box::new(ConcreteHandler3 {
        next: None,
    });

    handler2.set_next(Some(handler3));
    handler1.set_next(Some(handler2));
   

    handler1.handle("2");
    handler1.handle("1");
    handler1.handle("3");
    handler1.handle("4");

}


type THandler = Option<Box<dyn Hangler>>;


struct ConcreteHandler1 {
    next: THandler,
}

struct ConcreteHandler2 {
    next: THandler,
}

struct ConcreteHandler3 {
    next: THandler,
}

impl Hangler for ConcreteHandler1 {
    fn get_next(&mut self) -> &mut THandler {
        &mut self.next
    }
    fn handle(&mut self, message: &str) {
        if message == "1" {
            println!("Handler1");
        } else {
            self.next_handle(message);
        }
    }

}

impl Hangler for ConcreteHandler2 {
    fn get_next(&mut self) -> &mut THandler {
        &mut self.next
    }
    fn handle(&mut self, message: &str) {
        if message == "2" {
            println!("Handler2");
        } else {
            self.next_handle(message);
        }
    }
}

impl Hangler for ConcreteHandler3 {
    fn get_next(&mut self) -> &mut THandler {
        &mut self.next
    }
    fn handle(&mut self, message: &str) {
        if message == "3" {
            println!("Handler3");
        } else {
            self.next_handle(message);
            
        }
    }
}





trait Hangler {
    fn set_next(&mut self, handler: THandler) {
        let next = self.get_next();
        *next = handler;
    }

    fn next_handle(&mut self, message: &str) {
        match self.get_next() {
            Some(n) => {
                n.handle(message)
            },
            None => println!("None"),
        }
    }

    fn handle(&mut self, message: &str);

    fn get_next(&mut self) -> &mut THandler;
}


