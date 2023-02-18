use aeonetica_engine::register;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;

register!(client::CoreModClient{}, server::CoreModServer{});