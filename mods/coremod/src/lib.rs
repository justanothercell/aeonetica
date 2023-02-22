use aeonetica_engine::register;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;
pub mod common;

register!(client::CoreModClient{}, server::CoreModServer{});