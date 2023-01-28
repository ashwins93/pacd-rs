use std::{fs, path::Path};

use liquid::ValueView;
use log::error;

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
