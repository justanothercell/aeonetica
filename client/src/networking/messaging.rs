use aeonetica_server::ecs::Engine;

pub struct ClientMessenger {

}

impl ClientMessenger {
    pub fn new() {

    }
    pub fn register_server_receiver<F: Fn(&Id, &mut Engine, &Vec<u8>)>(&mut self, f: F) {}
}