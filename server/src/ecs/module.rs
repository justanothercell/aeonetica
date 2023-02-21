pub trait Module {
    fn init(&mut self) {}
    fn start(&self) {}
    fn tick(&self) {}
}

