use aeonetica_engine::register;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;
pub mod common_client;

register!(client::CoreModClient{}, server::CoreModServer{});