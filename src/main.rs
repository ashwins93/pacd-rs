use clap::{Args, Parser};
use log::error;
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::{error::Error, time::Duration};

use pacd::{
    packer::{Packer, PackerConfig},
    Config, SiteGenerator,
};

#[derive(Parser, Debug, Clone)]
#[command(name = "pacd")]
enum PacdCli {
    #[command(name = "build")]
    Builder(BuildArgs),
    #[command(name = "pack")]
    Packer(PackerArgs),
}

/// A static site generator based on shopify liquid
#[derive(Debug, Args, Clone)]
#[command(version, author, about, long_about = None)]
struct BuildArgs {
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

/// Pack your template files into an archive
#[derive(Debug, Args, Clone)]
#[command(version, author, about, long_about = None)]
struct PackerArgs {
    /// The path for output
    #[arg(short, long, default_value = "./build.tar.gz")]
    output_path: std::path::PathBuf,

    /// Path to the source files
    template_dir: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let cmd = PacdCli::parse();

    match cmd {
        PacdCli::Builder(args) => {
            let config = Config {
                src_path: &args.site_dir,
                dest_path: &args.output_dir,
                data_path: &args.data_path,
            };

            if args.watch {
                println!("Watching for file changes in {}", args.site_dir.display());

                match SiteGenerator::build(config).and_then(|mut s| s.generate()) {
                    Ok(_) => println!("Build successful"),
                    Err(e) => {
                        error!("Build failed: {:#?}", e);
                    }
                }
                watch_changes(&args.site_dir, || {
                    let config = Config {
                        src_path: &args.site_dir,
                        dest_path: &args.output_dir,
                        data_path: &args.data_path,
                    };
                    match SiteGenerator::build(config).and_then(|mut s| s.generate()) {
                        Ok(_) => println!("Build successful"),
                        Err(e) => {
                            error!("Build failed: {:#?}", e);
                        }
                    }
                    Ok(())
                })?;
            } else {
                SiteGenerator::build(config)?.generate()?;
            }
        }
        PacdCli::Packer(args) => {
            let packer = Packer::new(PackerConfig {
                src_path: &args.template_dir,
                dest_path: &args.output_path,
            });

            packer.pack()?;
        }
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
