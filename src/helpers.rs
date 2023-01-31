use std::{
    ffi::OsStr,
    fs::{self, File},
    path::Path,
};

use flate2::read::GzDecoder;
use liquid::ValueView;
use log::{debug, error};
use std::io::Read;
use tar::Archive;

use crate::errors::PacdError;

pub fn create_dir_for_path(filepath: &Path) -> Result<(), PacdError> {
    filepath
        .parent()
        .filter(|p| !p.exists())
        .map_or(Ok(()), |parent| {
            fs::create_dir_all(parent).map_err(|e| {
                error!(target: "create_dir_for_path", "Error creating directory {e}");
                PacdError::DestCreationError(parent.display().to_string())
            })
        })
}

pub fn get_id_string(obj: &dyn ValueView, coll_name: &str) -> Result<String, PacdError> {
    obj.as_object()
        .and_then(|o| o.get("id"))
        .ok_or(PacdError::NoIDField(coll_name.to_string()))?
        .as_scalar()
        .map(|s| s.to_kstr().into_string())
        .ok_or(PacdError::IDParseError(coll_name.to_string()))
}

fn unpack_archive_to<R: Read>(mut arch: Archive<R>, dst: &Path) -> Result<(), PacdError> {
    arch.unpack(dst).map_err(|e| {
        error!(target: "unpack_archive_to", "Error unpacking archive {e}");
        PacdError::DeflateError(dst.display().to_string())
    })
}

pub fn unpack_archive(file_src: &Path, dst: &Path) -> Result<(), PacdError> {
    debug!(target: "unpack_archive", "Unpacking archive to {dst}", dst = dst.display());
    let ext = file_src.extension().unwrap_or_else(|| {
        error!(target: "unpack_archive", "Error getting extension for {file}", file = file_src.display());
        OsStr::new("") })
    ;
    let file = File::open(file_src).map_err(|e| {
        error!(target: "unpack_archive", "Error opening file {file} {e}", file = file_src.display());
        PacdError::SrcReadError(file_src.display().to_string())
    })?;

    if ext == "tar" {
        debug!(target: "unpack_archive", "Reading from tar file");
        let arch = Archive::new(file);
        unpack_archive_to(arch, dst)?;
    } else if ext == "gz" {
        debug!(target: "unpack_archive", "Reading from tar.gz file");
        let gz = GzDecoder::new(file);
        let arch = Archive::new(gz);
        unpack_archive_to(arch, dst)?;
    }

    Ok(())
}
