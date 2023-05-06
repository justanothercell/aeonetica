use aeonetica_engine::register;

mod client;
mod server;
mod common;

register!(client::{{MOD_NAME_CAPITALIZED}}ModClient::new(), server::{{MOD_NAME_CAPITALIZED}}ModServer::new());