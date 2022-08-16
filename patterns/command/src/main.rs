fn main() {
    
    let invoker = Invoker::new();

    invoker.open();
    invoker.close();

}


trait Command {
    fn execute(&self);
}

struct Invoker {
    command_open: Box<dyn Command>,
    command_close: Box<dyn Command>,
}

impl Invoker {

    fn new() -> Self {
        Self {
            command_open: Box::new(CommandOpen),
            command_close: Box::new(CommandClose),
        }
    }


    fn close(&self) {
        self.command_close.execute();
    }

    fn open(&self) {
        self.command_open.execute();
    }
}


struct CommandOpen;



impl Command for CommandOpen {
    fn execute(&self) {
        println!("Open");
    }
}


struct CommandClose;



impl Command for CommandClose {

    

    fn execute(&self) {
        println!("Close");
    }
}





