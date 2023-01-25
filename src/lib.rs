use std::{error::Error, fs, io::Write, path::Path};

use liquid::{ParserBuilder, Template};
use regex::Regex;
use serde_json::Value;
use walkdir::WalkDir;

pub struct SingleOutputConfig<'a> {
    template: &'a Template,
    output_filename: &'a str,
    data: &'a Value,
}

pub struct CollectionOutputConfig<'a> {
    template: &'a Template,
    collection_name: &'a str,
    output_path: &'a Path,
    data: &'a Value,
}

pub fn build_site(src_path: &str, dest_path: &str) -> Result<(), Box<dyn Error>> {
    let re = Regex::new("^\\[(.*)\\]$").expect("Incorrect regex config");
    let parser = ParserBuilder::with_stdlib().build().unwrap();
    let v: Value = serde_json::from_str(
        r#"{
        "account": {
            "name": "Ashwin"
        },
        "collection": [
            { "name": "one", "id": "one" },
            { "name": "two", "id": "two" }
        ]
    }"#,
    )?;

    for entry in WalkDir::new(src_path) {
        let entry = entry.expect("Unable to traverse path");
        let path = entry.path();
        if path.is_file() {
            // new file path
            let output_path = Path::new(dest_path).join(path.strip_prefix(src_path)?);

            create_dir_for_path(&output_path)?;

            let mut output_filename = output_path.as_path().display().to_string();
            println!("Creating new file {}", &output_filename);

            let contents = match fs::read_to_string(path) {
                Ok(s) => s,
                Err(_) => return Err("Cannot open file".into()),
            };

            let ext = path.extension().unwrap_or_default();

            if ext == "liquid" {
                let template = match parser.parse(&contents) {
                    Ok(t) => t,
                    Err(_) => return Err("Unable to parse template".into()),
                };

                let coll_name = path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .and_then(|stem| re.captures(stem));

                match coll_name {
                    Some(captures) => {
                        let config = CollectionOutputConfig {
                            template: &template,
                            output_path: output_path.as_path(),
                            data: &v,
                            collection_name: captures
                                .get(1)
                                .ok_or("Unable to find collection name")?
                                .as_str(),
                        };
                        create_collection_output(&config)?;
                    }
                    None => {
                        output_filename.truncate(output_filename.len() - ".liquid".len());
                        output_filename.push_str(".html");
                        let config = SingleOutputConfig {
                            template: &template,
                            output_filename: &output_filename,
                            data: &v,
                        };
                        create_single_output(&config)?;
                    }
                };
            }
        }
    }
    Ok(())
}

fn create_dir_for_path(filepath: &Path) -> Result<(), Box<dyn Error>> {
    if !filepath.parent().unwrap().exists() {
        fs::create_dir_all(filepath.parent().unwrap())?;
    }

    Ok(())
}

fn create_single_output(config: &SingleOutputConfig) -> Result<(), Box<dyn Error>> {
    let globals = liquid::to_object(config.data)?;
    let contents = match config.template.render(&globals) {
        Ok(contents) => contents,
        Err(e) => return Err(Box::new(e.context("cause", "Attempt to render template"))),
    };
    let mut file = match fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(config.output_filename)
    {
        Ok(file) => file,
        Err(e) => {
            return Err(format!(
                "cannot create output file '{}': {}",
                config.output_filename, e
            )
            .into())
        }
    };
    if file.write_all(contents.as_bytes()).is_err() {
        return Err("unable to write to file".into());
    }
    Ok(())
}

fn create_collection_output(config: &CollectionOutputConfig) -> Result<(), Box<dyn Error>> {
    let key = config.collection_name;
    let list = config.data.get(key).ok_or("Collection key not found")?;
    let list = list.as_array().ok_or("Unable to parse collection")?;

    for val in list {
        let id = val
            .get("id")
            .ok_or("Cannot find ID for collection")?
            .as_str()
            .ok_or("Cannot parse ID as string")?;
        let new_filename = format!("{}.html", id);
        let full_path = Path::new(config.output_path.parent().unwrap()).join(new_filename);
        let output_filename = full_path.to_str().ok_or("Unable to create new path")?;

        let mut val_config = config.data.clone();

        val_config
            .as_object_mut()
            .ok_or("Invalid data")?
            .insert(format!("{}_value", config.collection_name), val.clone());

        let single_config = SingleOutputConfig {
            template: config.template,
            output_filename,
            data: &val_config,
        };

        create_single_output(&single_config)?
    }

    Ok(())
}
