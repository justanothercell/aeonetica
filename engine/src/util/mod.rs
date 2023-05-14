pub mod id_map;
pub mod nullable;

use std::any::{type_name};

use std::fmt::Display;
use std::path::Path;
use std::fs::File;
use core::hash::Hasher;
#[allow(deprecated)]
use std::hash::SipHasher;
use crate::error::*;
use crate::error::builtin::IOError;
use crate::{Id, TypeId};

pub fn unzip_archive<R: std::io::Read + std::io::Seek, P: AsRef<Path> + Display>(zip: R, dest_dir: P) -> ErrorResult<()> {
    let mut archive = zip::read::ZipArchive::new(zip)
        .map_err(|e| Error::new(IOError(format!("could not read zip file: {e}")), Fatality::FATAL, true))?;
    std::fs::create_dir_all(&dest_dir).expect("unable to create directory");
    for i in 0..archive.len() {
        let mut f = archive.by_index(i)
            .map_err(|e| Error::new(IOError(format!("could not read zip file: {e}")), Fatality::FATAL, true))?;
        let full_path = f.enclosed_name().unwrap().to_str().unwrap();
        if f.is_dir() {
            std::fs::create_dir_all(&format!("{dest_dir}/{full_path}"))?;
        } else {
            if let Some(p) = f.enclosed_name().unwrap().parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&format!("{dest_dir}/{full_path}"))?;
            std::io::copy(&mut f, &mut outfile)?;
        }
    }
    Ok(())
}

#[cfg(debug_assertions)]
mod debug_id {
    use std::sync::Mutex;
    use lazy_static::lazy_static;
    use crate::Id;
    use crate::util::id_map::IdMap;

    lazy_static! {
        pub(crate) static ref ID_TYPE_MAP: Mutex<IdMap<String>> = Default::default();
    }

    impl Id {
        pub fn info(&self) -> String {
            ID_TYPE_MAP.lock().unwrap().get(self).map(|s|s.to_string()).unwrap_or(self.to_string())
        }
    }
}
#[cfg(not(debug_assertions))]
mod debug_id {
    use crate::Id;

    impl Id {
        pub fn info(&self) -> String {
            self.to_string()
        }
    }
}

pub fn type_to_id<T>() -> TypeId {
    #[allow(deprecated)]
    let mut s = SipHasher::new();
    s.write(type_name::<T>().as_bytes());
    let id = Id::from_u64(s.finish());
    #[cfg(debug_assertions)]
    debug_id::ID_TYPE_MAP.lock().unwrap().insert(id, type_name::<T>().to_string());
    id
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Either<A, B> {
    Left(A),
    Right(B)
}

