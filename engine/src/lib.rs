#![feature(const_hash)]
#![feature(const_type_name)]
#![feature(hashmap_internals)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

use std::fmt::{Debug, Display, Formatter};
pub use nanoserde;
pub use libloading;
pub use chrono;
use nanoserde::{DeBin, DeRon, SerBin, SerRon};
pub use sha2;
use uuid::Uuid;

pub mod networking;
pub mod error;
pub mod util;

#[derive(Copy, Clone, SerBin, DeBin, SerRon, DeRon, Eq, PartialEq, Hash)]
pub struct Id([u8;16]);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4().into_bytes())
    }

    pub const fn from_bytes(b: [u8;16]) -> Self {
        Self(b)
    }

    pub const fn into_bytes(self) -> [u8;16] {
        self.0
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Uuid::from_bytes(self.0))
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub const ENGINE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MAX_CLIENT_TIMEOUT: u128 = 5000; // 5s

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
        eprintln!()
    };
    ($($arg:tt)*) => {
        eprintln!("[@{}]\n{} [{} - ERR]: {}", format!("{}:{}:{}", file!(), line!(), column!()), $crate::chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"), env!("CARGO_PKG_NAME"), format!($($arg)*))
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