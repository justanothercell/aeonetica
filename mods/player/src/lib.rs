use aeonetica_engine::register;

mod client;
pub mod server;

register!(client::PlayerModClient{}, server::PlayerModServer{});