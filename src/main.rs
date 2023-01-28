use std::error::Error;

use pacd::SiteGenerator;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let src_path = "./site";
    let dest_path = "./build";
    let data_path = "./site/data.json";

    SiteGenerator::build(src_path, dest_path, data_path)?.generate()?;
    Ok(())
}
