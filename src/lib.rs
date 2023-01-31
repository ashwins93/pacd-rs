use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, File},
    io::{BufReader, Write},
    path::Path,
};

use liquid::{model::ScalarCow, ObjectView, ParserBuilder, Template, ValueView};
use log::{debug, error};
use regex::Regex;
use tempfile::TempDir;
use walkdir::WalkDir;

pub mod config;
pub mod errors;
pub mod helpers;
pub mod packer;

pub use crate::config::Config;
pub use crate::errors::PacdError;

pub struct SiteGenerator<'a> {
    src_path: &'a Path,
    dest_path: &'a Path,
    parser: liquid::Parser,
    globals: HashMap<String, liquid::model::Value>,
    coll_pattern: regex::Regex,
}

impl SiteGenerator<'_> {
    pub fn build(config: Config) -> Result<SiteGenerator, PacdError> {
        let data = config.data_path;
        let src = config.src_path;
        let dest = config.dest_path;

        // create a parser
        let parser = ParserBuilder::with_stdlib().build().map_err(|e| {
            error!(target: "SiteGenerator::build", "Error building parser {e}");
            PacdError::CouldNotBuildParser
        })?;

        // get the data bindings from file
        let file = File::open(Path::new(data)).map_err(|e| {
            error!(target: "SiteGenerator::build", "Error opening file {e}");
            PacdError::DataParseError(data.display().to_string())
        })?;
        let rdr = BufReader::new(file);
        let globals: HashMap<String, liquid::model::Value> =
            serde_json::from_reader(rdr).map_err(|e| {
                error!(target: "SiteGenerator::build", "Serde parse failed {e}");
                PacdError::DataParseError(data.display().to_string())
            })?;

        // pattern for collection types
        let coll_pattern =
            Regex::new("^\\[(.*)\\]$").expect("Incorrect regex config. Contact library author.");

        Ok(SiteGenerator {
            src_path: src,
            dest_path: dest,
            parser,
            globals,
            coll_pattern,
        })
    }

    pub fn generate(&mut self) -> Result<(), PacdError> {
        let ext = self.src_path.extension().unwrap_or(OsStr::new(""));
        debug!(target: "SiteGenerator::generate", "Extension: {ext:?}");

        let tmpdir: TempDir;
        let mut src_path = self.src_path;

        if ext == "tar" || ext == "gz" {
            tmpdir = tempfile::tempdir().map_err(|e| {
                error!(target: "SiteGenerator::generate", "Error creating temp dir {e}");
                PacdError::DeflateError(self.src_path.display().to_string())
            })?;
            src_path = tmpdir.path();

            helpers::unpack_archive(self.src_path, src_path)?;
        }

        for entry in WalkDir::new(src_path) {
            let entry = entry.map_err(|e| {
                error!(target: "SiteGenerator::generate", "walk error {e}");
                PacdError::TraverseError
            })?;
            let path = entry.path();
            debug!(target: "SiteGenerator::generate", "Processing path {path}", path = path.display());
            if path.is_file() {
                let output_path =
                    Path::new(&self.dest_path).join(path.strip_prefix(src_path).unwrap_or(path));
                self.build_file(path, &output_path)?;
            }
        }

        Ok(())
    }

    fn build_file(&mut self, src_path: &Path, output_path: &Path) -> Result<(), PacdError> {
        debug!(target: "SiteGenerator::prep_src_file", "Processing file {path}", path = src_path.display());
        // new file path

        helpers::create_dir_for_path(output_path)?;

        let ext = src_path.extension().unwrap_or_else(|| OsStr::new(""));

        if ext == "liquid" {
            self.transform_file(src_path, output_path)?;
        } else {
            fs::copy(src_path, output_path).map_err(|e| {
                error!(target: "SiteGenerator::prep_src_file", "Copy error {e}");
                PacdError::DestCreationError(output_path.display().to_string())
            })?;
        }
        Ok(())
    }

    fn transform_file(&mut self, input_path: &Path, output_path: &Path) -> Result<(), PacdError> {
        let output_filename = output_path.display().to_string();

        let template = self.parser.parse_file(input_path).map_err(|e| {
            error!(target: "SiteGenerator::generate", "error parsing template {e}");
            PacdError::CouldNotParseTemplate(input_path.display().to_string())
        })?;

        let coll_name = input_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .and_then(|stem| self.coll_pattern.captures(stem));

        match coll_name {
            Some(captures) => {
                let config = CollectionOutputConfig {
                    template: &template,
                    output_path,
                    collection_name: captures.get(1).unwrap().as_str(),
                };
                self.create_collection_output(&config)?;
            }
            None => {
                let output_filename = Path::new(&output_filename).with_extension("html");
                let config = SingleOutputConfig {
                    template: &template,
                    output_filename: &output_filename,
                    locals: HashMap::new(),
                };

                self.create_single_output(&config)?;
            }
        };
        Ok(())
    }

    fn create_single_output(&self, config: &SingleOutputConfig) -> Result<(), PacdError> {
        let globals = PageData {
            data: &self.globals,
            page: &config.locals,
        };

        let contents = config.template.render(&globals).map_err(|e| {
            error!("Cannot render template {:?}", e);
            PacdError::CouldNotRenderFile(config.output_filename.display().to_string())
        })?;
        println!("Creating file {}", config.output_filename.display());

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(config.output_filename)
            .map_err(|e| {
                error!(
                    "Cannot create output file {}: {e}",
                    config.output_filename.display()
                );
                PacdError::DestCreationError(config.output_filename.display().to_string())
            })?;

        file.write_all(contents.as_bytes())
            .map_err(|e| {
                error!(target: "SiteGenerator::create_single_output", "writing output to file failed {e}");
                PacdError::DestCreationError(config.output_filename.display().to_string())
        })
    }

    fn create_collection_output(
        &mut self,
        config: &CollectionOutputConfig,
    ) -> Result<(), PacdError> {
        let key = config.collection_name;
        let list = self
            .globals
            .get(key)
            .ok_or(PacdError::CollectionKeyNotFound(key.to_string()))?
            .as_view()
            .as_array()
            .ok_or(PacdError::NoListAvailable(key.to_string()))?;

        for (idx, val) in list.values().enumerate() {
            let id = helpers::get_id_string(val, key)?;
            let new_filename = format!("{id}.html");
            let full_path = Path::new(config.output_path.parent().unwrap()).join(new_filename);

            let mut locals = HashMap::new();
            let val = ScalarCow::new(idx as u32).to_value();
            locals.insert("current_index".to_string(), val);

            let single_config = SingleOutputConfig {
                template: config.template,
                output_filename: &full_path,
                locals,
            };

            self.create_single_output(&single_config)?
        }

        Ok(())
    }
}

struct SingleOutputConfig<'a> {
    template: &'a Template,
    output_filename: &'a Path,
    locals: HashMap<String, liquid::model::Value>,
}

struct CollectionOutputConfig<'a> {
    template: &'a Template,
    collection_name: &'a str,
    output_path: &'a Path,
}

#[derive(Debug, ObjectView, ValueView)]
struct PageData<'a> {
    data: &'a HashMap<String, liquid::model::Value>,
    page: &'a HashMap<String, liquid::model::Value>,
}
