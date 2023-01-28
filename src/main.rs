use clap::Parser;
use std::error::Error;

use pacd::SiteGenerator;

/// A static site generator based on shopify liquid
#[derive(Debug, Parser)]
struct CliArgs {
    /// The path for output
    #[arg(short, long, default_value = "./build")]
    output_dir: std::path::PathBuf,

    /// Path to the JSON data
    #[arg(short, long, default_value = "./data.json")]
    data_path: std::path::PathBuf,

    /// Path to the source files
    site_dir: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = CliArgs::parse();

    let src_path = args.site_dir;
    let dest_path = args.output_dir;
    let data_path = args.data_path;

    SiteGenerator::build(&src_path, &dest_path, &data_path)?.generate()?;
    Ok(())
}
