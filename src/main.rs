use clap::Parser;
use log::error;
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::{error::Error, time::Duration};

use pacd::SiteGenerator;

/// A static site generator based on shopify liquid
#[derive(Debug, Parser, Clone)]
struct CliArgs {
    /// The path for output
    #[arg(short, long, default_value = "./build")]
    output_dir: std::path::PathBuf,

    /// Path to the JSON data
    #[arg(short, long, default_value = "./data.json")]
    data_path: std::path::PathBuf,

    /// Watch for directory changes
    #[arg(short, long)]
    watch: bool,

    /// Path to the source files
    site_dir: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = CliArgs::parse();

    if args.watch {
        println!("Watching for file changes in {}", args.site_dir.display());
        SiteGenerator::build(&args.site_dir, &args.output_dir, &args.data_path)?.generate()?;
        watch_changes(&args.site_dir, || {
            SiteGenerator::build(&args.site_dir, &args.output_dir, &args.data_path)?.generate()?;
            Ok(())
        })?;
    } else {
        SiteGenerator::build(&args.site_dir, &args.output_dir, &args.data_path)?.generate()?;
    }

    Ok(())
}

fn watch_changes<F>(path: &std::path::Path, f: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce() -> Result<(), Box<dyn Error>> + Copy,
{
    let (tx, rx) = crossbeam::channel::bounded(1);

    let mut debouncer = new_debouncer(Duration::from_millis(500), None, tx)?;

    debouncer.watcher().watch(path, RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(e) => {
                let paths: Vec<_> = e.iter().map(|e| &e.path).collect();

                println!("file(s) changed {paths:?}\nRebuilding...");
                if e.iter().any(|e| e.kind == DebouncedEventKind::Any) {
                    f()?;
                }
            }
            Err(e) => error!("watch error {:#?}", e),
        };
    }

    Ok(())
}
