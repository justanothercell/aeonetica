use aeonetica_engine::register;

mod client;
mod server;

register!(client::PlayerModClient{}, server::PlayerModServer{});