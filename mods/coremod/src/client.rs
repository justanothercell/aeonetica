use aeonetica_client::ClientMod;
use aeonetica_engine::log;

pub struct CoreModClient {

}

impl ClientMod for CoreModClient {
    fn init(&mut self, _flags: &Vec<String>) {
        log!("hello from client core!")
    }
}