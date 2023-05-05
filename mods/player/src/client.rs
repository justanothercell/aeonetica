use aeonetica_client::ClientMod;
use aeonetica_client::networking::messaging::ClientHandle;
use aeonetica_engine::networking::messaging::ClientEntity;

pub struct PlayerModClient {

}

impl ClientMod for PlayerModClient {

}

pub struct PlayerHandle {
    
}

impl ClientEntity for PlayerHandle {}

impl ClientHandle for PlayerHandle {
    
}