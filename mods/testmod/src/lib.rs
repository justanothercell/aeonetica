use aeonetica_engine::register;

pub mod client;
pub mod server;

register!(client::CoreModClient{}, server::CoreModServer{});