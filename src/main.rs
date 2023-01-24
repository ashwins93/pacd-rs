use liquid::ParserBuilder;
use std::{fs, io::Write, path::Path};
use walkdir::WalkDir;
fn main() {
    let src_path = "./site";
    let dest_path = "./build";
    let parser = ParserBuilder::with_stdlib().build().unwrap();

    for entry in WalkDir::new(src_path) {
        let entry = entry.expect("Unable to traverse path");
        let path = entry.path();
        if path.is_file() {
            println!("{}", path.canonicalize().unwrap().display());

            // new file path
            let output_path = Path::new(dest_path).join(path.strip_prefix(src_path).unwrap());

            if !output_path.parent().unwrap().exists() {
                fs::create_dir_all(output_path.parent().unwrap()).expect("Unable to create dir");
            }

            let mut output_filename = output_path.as_path().display().to_string();
            println!("Creating new file {}", &output_filename);

            let mut contents = fs::read_to_string(path).expect("Unable to read file");

            if let Some(ext) = path.extension() {
                if ext == "liquid" {
                    let template = parser.parse(&contents).expect("Unable to parse file");

                    contents = template
                        .render(&liquid::object!({
                            "account": {
                                "name": "Ashwin"
                            }
                        }))
                        .expect("Unable to render");

                    output_filename.truncate(output_filename.len() - ".liquid".len());
                    output_filename.push_str(".html");
                }
            }

            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&output_filename)
                .unwrap();
            file.write_all(contents.as_bytes())
                .expect("unable to write to file");
        }
    }
}
