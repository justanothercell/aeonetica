use aeonetica_engine::{log};

use server::server;


fn main() {
	aeonetica_engine::enable_ansi_support::enable_ansi_support().unwrap_or_else(|_| eprintln!("ansi not supported in this console"));
    // cargo run -- 0.0.0.0:6090
    let mut args: Vec<_> = std::env::args().skip(1).collect();
    log!("started server with args {args:?}");
    if args.is_empty() {
        args.push("0.0.0.0:6090".to_string());
        //let e = AError::new(AET::ValueError(format!("expected command line arg ip:port>, got {}", args.len())));
        //e.log_exit();
    }
    server::run(&args[0]);
}