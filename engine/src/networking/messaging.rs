pub trait ClientHandle {
    fn init(&mut self) {}
    fn start(&mut self, messenger: &mut ClientMessenger) {}
    fn remove(&mut self, messenger: &mut ClientMessenger) {}
}