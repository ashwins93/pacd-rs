use std::path::Path;

pub struct Config<'a> {
    pub src_path: &'a Path,
    pub dest_path: &'a Path,
    pub data_path: &'a Path,
}
