pub trait ClientHandle {
    fn init(&mut self) {}
    fn start(&mut self, messenger: &mut ClientMessenger) {}
    fn remove(&mut self, messenger: &mut ClientMessenger) {}
}

pub struct ClientMessenger {

}

impl ClientMessenger {
    pub fn register_server_receiver(&mut self) {}
}