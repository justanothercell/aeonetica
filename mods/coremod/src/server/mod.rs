use aeonetica_engine::log;
use aeonetica_server::ServerMod;

pub struct CoreModServer {

}

impl ServerMod for CoreModServer {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from client core!")
    }
}