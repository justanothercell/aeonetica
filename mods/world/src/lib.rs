#![feature(generators)]
#![feature(generator_trait)]

use aeonetica_engine::register;

pub mod client;
pub mod server;

register!(client::TestModClient{}, server::TestModServer{});