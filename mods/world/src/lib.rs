use aeonetica_engine::register;

pub mod client;
pub mod server;
pub mod common;
pub mod tiles;

register!(client::WorldModClient{}, server::WorldModServer::new());
