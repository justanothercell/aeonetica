use aeonetica_engine::{log};
use aeonetica_engine::error::{AError, AET};
use server::server;


fn main() {
    // cargo run -- 0.0.0.0:6090
    let args: Vec<_> = std::env::args().skip(1).collect();
    log!("started server with args {args:?}");
    if args.is_empty() {
        let e = AError::new(AET::ValueError(format!("expected command line arg ip:port>, got {}", args.len())));
        e.log_exit();
    }
    server::run(&args[0]);
}