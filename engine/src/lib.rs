pub use nanoserde;
pub use libloading;
pub use chrono;
pub use uuid;
pub use sha2;

pub mod networking;
pub mod error;
pub mod util;

pub type Id = [u8;16];

pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_export]
macro_rules! log {
    () => {
        println!()
    };
    ($($arg:tt)*) => {
        println!("{} [{} - LOG]: {}", $crate::chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"), env!("CARGO_PKG_NAME"), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_raw {
    () => {
        println!()
    };
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_err {
    () => {
        println!()
    };
    ($($arg:tt)*) => {
        println!("[@{}]\n{} [{} - ERR]: {}", format!("{}:{}:{}", file!(), line!(), column!()), $crate::chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"), env!("CARGO_PKG_NAME"), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! register {
    ($client_mod:expr, $server_mod:expr) => {
        use aeonetica_client as _a_c;
        use aeonetica_server as _a_s;
        #[no_mangle]
        #[cfg(feature = "client")]
        pub fn _create_mod_client() -> Box<dyn _a_c::ClientMod> {
            Box::new($client_mod)
        }
        #[no_mangle]
        #[cfg(feature = "server")]
        pub fn _create_mod_server() -> Box<dyn _a_s::ServerMod> {
            Box::new($server_mod)
        }
    };
}