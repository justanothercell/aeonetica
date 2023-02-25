use std::any::{type_name};

use std::fmt::Display;
use std::path::Path;
use std::fs::File;
use core::hash::Hasher;
use std::hash::SipHasher13;
use crate::error::{AError, AET};
use crate::Id;


pub fn unzip_archive<R: std::io::Read + std::io::Seek, P: AsRef<Path> + Display>(zip: R, dest_dir: P) -> Result<(), AError>{
    let mut archive = zip::read::ZipArchive::new(zip)
        .map_err(|e| AError::new(AET::IOError(format!("could not read zip file: {e}"))))?;
    std::fs::create_dir_all(&dest_dir).expect("unable to create directory");
    for i in 0..archive.len() {
        let mut f = archive.by_index(i)
            .map_err(|e| AError::new(AET::IOError(format!("could not read zip file: {e}"))))?;
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

pub const fn type_to_id<T>() -> Id {
    #[allow(deprecated)]
    let mut s = SipHasher13::new();
    s.write(type_name::<T>().as_bytes());
    let [a, b, c, d,e , f, g, h] = s.finish().to_le_bytes();
    s.write(type_name::<T>().as_bytes());
    let [i, j, k, l, m, n, o, p] = s.finish().to_le_bytes();
    Id::from_bytes([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
}