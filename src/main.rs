use std::{
    collections::HashMap,
    fs::{self, metadata},
    io::Error,
    path::PathBuf,
    process,
};

use ansi_term::{Colour::Red, Style};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[clap(name = "dedup")]
#[clap(author = "rzru <razzaru@yandex.ru>")]
#[clap(about = "Deletes duplicated files in chosen folder")]
#[clap(version, long_about = None)]
struct Cli {
    /// Path to directory
    path: String,

    /// Process inner directories (default = false)
    #[clap(short, long)]
    recursive: bool,

    /// Delete duplicate files even if they are in different directories (default = false)
    #[clap(short, long)]
    ignore_dir: bool,
}

fn main() {
    let cli = Cli::parse();

    let result = run_app(cli);

    if let Err(error) = result {
        eprintln!("{}: {}", Red.bold().paint("ERROR"), error);
        process::exit(1);
    }

    let result = result.unwrap();
    print_bold(format!("Successfully removed {} files:", result.len()));
    result.iter().for_each(|path| {
        print_bold(format!("{:?}", path));
    });
}

fn run_app(config: Cli) -> Result<Vec<PathBuf>, Error> {
    let mut deleted = vec![];
    let mut directories = vec![PathBuf::from(config.path)];
    let mut checksums: HashMap<u32, PathBuf> = HashMap::new();

    while let Some(dir) = directories.pop() {
        let count = fs::read_dir(&dir)?.count() as u64;

        print_bold(format!("Processing {} files in {:?}", count, &dir));
        let progress_bar = get_progress(count);

        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if metadata(&path)?.is_dir() {
                progress_bar.inc(1);
                if config.recursive {
                    directories.insert(0, path);
                }

                continue;
            }

            let buf = fs::read(&path)?;
            let checksum = crc32fast::hash(&buf);

            if let Some(_) = checksums.get(&checksum) {
                fs::remove_file(&path)?;
                deleted.push(path);
            } else {
                checksums.insert(checksum, path);
            }
            progress_bar.inc(1);
        }

        if !config.ignore_dir {
            checksums = HashMap::new();
        }

        progress_bar.finish();
    }

    Ok(deleted)
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

fn print_bold(text: String) {
    println!("{}", Style::new().bold().paint(format!("> {}", text)));
}
