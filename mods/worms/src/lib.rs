use aeonetica_engine::register;

mod client;
mod server;

register!(client::WormsModClient::new(), server::WormsModServer::new());