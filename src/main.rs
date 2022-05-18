use std::{
    collections::HashMap,
    fs::{self, metadata},
    io::Error,
    path::PathBuf,
    process,
};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

extern crate pretty_env_logger;

#[macro_use]
extern crate log;

#[derive(Parser)]
#[clap(name = "dedup")]
#[clap(author = "rzru <razzaru@yandex.ru>")]
#[clap(about = "Deletes duplicated files in chosen folder")]
#[clap(version, long_about = None)]
struct Cli {
    path: String,
}

fn main() {
    let cli = Cli::parse();
    pretty_env_logger::init();

    let result = run_app(cli);

    if let Err(error) = result {
        error!("Error: {}", error);
        process::exit(1);
    }

    info!("Successfully removed {} files", result.unwrap());
}

fn run_app(config: Cli) -> Result<u32, Error> {
    let mut delete_count = 0;
    let mut checksums: HashMap<u32, PathBuf> = HashMap::new();
    let count = fs::read_dir(&config.path)?.count() as u64;

    info!("Processing {} files", count);
    let progress_bar = get_progress(count);

    for entry in fs::read_dir(config.path)? {
        let path = entry?.path();
        if metadata(&path)?.is_dir() {
            progress_bar.inc(1);
            continue;
        }

        let buf = fs::read(&path)?;
        let checksum = crc32fast::hash(&buf);

        if let Some(_) = checksums.get(&checksum) {
            fs::remove_file(&path)?;
            delete_count += 1;
            info!("Successfully removed duplicate: {:?}", path);
        } else {
            checksums.insert(checksum, path);
        }
        progress_bar.inc(1);
    }

    progress_bar.finish();
    Ok(delete_count)
}

fn get_progress(count: u64) -> ProgressBar {
    let progress_bar = ProgressBar::new(count);
    let sty = ProgressStyle::default_bar()
        .template(
            "{spinner:.cyan} [{elapsed_precise}] {wide_bar:.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .progress_chars("##-");
    progress_bar.set_style(sty);

    progress_bar
}
