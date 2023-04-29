#![feature(const_hash)]
#![feature(const_type_name)]
#![feature(hashmap_internals)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(let_chains)]

#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests;

use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hasher, SipHasher};
pub use nanoserde;
pub use libloading;
pub use chrono;
use nanoserde::{DeBin, DeRon, SerBin, SerRon};
pub use sha2;
use uuid::Uuid;
pub extern crate colored;

pub mod networking;
pub mod error;
pub mod util;
pub mod collections;

#[derive(Copy, Clone, SerBin, DeBin, SerRon, DeRon, Eq, PartialEq, Hash)]
pub struct Id([u8;8]);

pub type ClientId = Id;
pub type EntityId = Id;
pub type TypeId = Id;

impl Id {
    #[inline]
    pub fn new() -> Self {
        let mut hasher = SipHasher::default();
        hasher.write(&Uuid::new_v4().into_bytes());
        Self::from_u64(hasher.finish())
    }

    #[inline]
    pub const fn from_bytes(b: [u8;8]) -> Self {
        Self(b)
    }

    #[inline]
    pub const fn from_u64(b: u64) -> Self {
        Self(b.to_le_bytes())
    }

    #[inline]
    pub const fn into_bytes(self) -> [u8;8] {
        self.0
    }

    #[inline]
    pub const fn into_u64(self) -> u64 {
        u64::from_le_bytes(self.0)
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}{:X}-{:X}{:X}-{:X}{:X}-{:X}{:X}", self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7])
    }
}

impl Default for Id {
     fn default() -> Self {
         Self::new()
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
macro_rules! log_format {
    ($color:ident, $level:literal, $($arg:tt)*) => {
        $crate::colored::Colorize::$color(
            format!(
                "[@{}]\n{} [{} - {}]: {}",
                format!("{}:{}:{}",file!(), line!(), column!()),
                $crate::chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                env!("CARGO_PKG_NAME"),
                $level,
                format!($($arg)*)
            ).as_str()
        )
    }
}

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
        eprintln!("{}", $crate::log_format!(red, "ERR", $($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    () => {
        println!()
    };
    ($($arg:tt)*) => {
        eprintln!("{}", $crate::log_format!(bright_yellow, "WARN", $($arg)*))
    }
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