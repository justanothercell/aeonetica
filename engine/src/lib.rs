#![feature(const_hash)]
#![feature(const_type_name)]
#![feature(hashmap_internals)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(let_chains)]
#![feature(try_trait_v2)]
#![feature(const_replace)]
#![feature(const_refs_to_cell)]
#![feature(const_option_ext)]
#![feature(macro_metavar_expr)]
#![feature(generic_const_exprs)]

#![feature(test)]
#![feature(try_trait_v2_residual)]
extern crate test;

#[cfg(test)]
mod tests;

use std::fmt::{Debug, Display, Formatter};
#[allow(deprecated)]
use std::hash::{Hasher, SipHasher};
use std::sync::Mutex;
pub use nanoserde;
pub use libloading;
pub use chrono;
use nanoserde::{DeBin, DeRon, SerBin, SerRon};
pub use sha2;
use uuid::Uuid;
pub extern crate colored;
use lazy_static::lazy_static;

pub mod networking;
pub mod error;
pub mod util;
pub mod collections;
pub mod math;
pub mod time;

pub use enable_ansi_support;

pub type Color = [f32; 4];

#[derive(Copy, Clone, SerBin, DeBin, SerRon, DeRon, Eq, PartialEq, Hash)]
pub struct Id([u8;8]);

pub type ClientId = Id;
pub type EntityId = Id;
pub type TypeId = Id;

impl Id {
    #[inline]
    pub fn new() -> Self {
        #[allow(deprecated)]
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
        write!(f, "{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}-{:02X}{:02X}", self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7])
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
pub const MAX_CLIENT_TIMEOUT: u128 = 10000; // 10s
pub static MOD_TARGET: &str = const_format::concatcp!(std::env::consts::ARCH, "-", std::env::consts::FAMILY);

#[macro_export]
macro_rules! log_format {
    ($color:ident, $level:literal, $($arg:tt)*) => {
        format!(
            "\x1b[38;5;245m{}[{}@{}:{}]: {}",
            $crate::chrono::Local::now().format("[%H:%M:%S]"),
            env!("CARGO_PKG_NAME"),
            file!(), line!(),
            $crate::colored::Colorize::$color(format!($($arg)*).as_str())
        )
    }
}

lazy_static! {
    static ref PACK_LOG_COUNTER: Mutex<u32> = Mutex::new(0);
    static ref PACK_LOG_HASH: Mutex<u64> = Mutex::new(0);
}

pub fn pack_log(origin: String, message: String) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::io::{self, Write};
    let mut hasher = DefaultHasher::new();
    origin.hash(&mut hasher);
    let hash_value = hasher.finish();
    let p_hash = *PACK_LOG_HASH.lock().unwrap();
    if p_hash == hash_value {
        let mut log_count = PACK_LOG_COUNTER.lock().unwrap();
        *log_count += 1;
        print!("\r[and {} more]", log_count);
        let _ = io::stdout().flush();
    } else {
        stop_pack_log();
        *PACK_LOG_HASH.lock().unwrap() = hash_value;
        println!("{message}")
    }
}

pub fn stop_pack_log() {
    let mut p_hash = PACK_LOG_HASH.lock().unwrap();
    let mut log_count = PACK_LOG_COUNTER.lock().unwrap();
    if *p_hash != 0 {
        *p_hash = 0;
        if *log_count > 0 { println!() }
        *log_count = 0;
    }
}

#[macro_export]
macro_rules! log {
    () => {
        println!()
    };
    (PACK, $($args:tt)*) => {{
        $crate::pack_log(format!("{}:{}", file!(), line!()), $crate::log_format!(white, "PACK", $($args)*))
    }};
    (DEBUG, $($args:tt)*) => {{
        $crate::stop_pack_log();
        println!("{}", $crate::log_format!(cyan, "DEBUG", $($args)*))
    }};
    (WARN, $($args:tt)*) => {{
        $crate::stop_pack_log();
        println!("{}", $crate::log_format!(bright_yellow, "LOG", $($args)*))
    }};
    (ERROR, $($args:tt)*) => {{
        $crate::stop_pack_log();
        println!("{}", $crate::log_format!(red, "ERROR", $($args)*))
    }};
    ($($args:tt)*) => {{
        $crate::stop_pack_log();
        println!("{}", $crate::log_format!(white, "LOG", $($args)*))
    }};
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