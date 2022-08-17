fn main() {
    let mut context = Context::new();
    context.set_func(|a, b| a+b);
    println!("{}", context.execute(7, 4));
    context.set_func(|a, b| a-b);
    println!("{}", context.execute(7, 4));
    context.set_func(|a, b| a*b);
    println!("{}", context.execute(7, 4));
}


struct Context {
    func: Option<fn(isize, isize) -> isize>,
}

impl Context {

    fn new() -> Self {
        Self {
            func: None,
        }
    }

    fn set_func(&mut self, f: fn(isize, isize) -> isize) {
        self.func = Some(f);
    }

    fn execute(&self, a: isize, b: isize) -> isize {
        match self.func {
            Some(f) => f(a, b),
            None => 0,
        }
    }
}