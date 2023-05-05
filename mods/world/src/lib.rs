use aeonetica_engine::register;

mod client;
mod server;
mod common;

register!(client::WorldModClient{}, server::WorldModServer::new());