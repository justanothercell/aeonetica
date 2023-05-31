use aeonetica_engine::register;

mod client;
mod server;

pub use client::Debug;

register!(client::DebugModClient::new(), server::DebugModServer::new());