use aeonetica_engine::register;

pub(crate) mod client;
pub(crate) mod server;

register!(client::WormsModClient::new(), server::WormsModServer::new());