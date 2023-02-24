use std::any::TypeId;
use std::fmt::Display;
use std::path::Path;
use std::fs::File;
use crate::error::{AError, AET};


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

pub unsafe fn typeid_to_i64(id: TypeId) -> i64{
    std::mem::transmute(id)
}

pub unsafe fn i64_to_typeid(id: i64) -> TypeId{
    std::mem::transmute(id)
}