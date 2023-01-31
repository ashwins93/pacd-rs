use std::{fs, path::Path};

use flate2::{write::GzEncoder, Compression};
use log::error;
use tar::Builder;
use walkdir::WalkDir;

use crate::errors::PackerError;

pub struct Packer<'a> {
    src_path: &'a Path,
    dest_path: &'a Path,
}

pub struct PackerConfig<'a> {
    pub src_path: &'a Path,
    pub dest_path: &'a Path,
}

impl Packer<'_> {
    pub fn new(config: PackerConfig) -> Packer {
        let src = config.src_path;
        let dest = config.dest_path;

        Packer {
            src_path: src,
            dest_path: dest,
        }
    }

    pub fn pack(&self) -> Result<(), PackerError> {
        let file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(self.dest_path)
            .map_err(|e| {
                error!(target: "Packer::pack", "Error opening file {e}");
                PackerError::CouldNotCreateFile(self.dest_path.display().to_string())
            })?;
        let file = GzEncoder::new(file, Compression::best());

        let iter = WalkDir::new(self.src_path)
            .into_iter()
            .filter_map(|e| e.ok());

        let mut builder = Builder::new(file);

        for entry in iter {
            let output_path = entry
                .path()
                .strip_prefix(self.src_path)
                .unwrap_or(self.src_path);

            if !output_path.display().to_string().is_empty() {
                builder
                    .append_path_with_name(entry.path(), output_path)
                    .map_err(|e| {
                        error!(target: "Packer::pack", "Error packing file {e}");
                        PackerError::CouldNotWriteToFile(self.dest_path.display().to_string())
                    })?;
            }
        }

        Ok(())
    }
}
