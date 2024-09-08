use std::{fs::DirEntry, process};

use clap::Parser;
use kbt::{get_dir_entries, process_path};

#[derive(Parser)]
struct Cli {
    path: String,
}

fn get_file_name(entry: &DirEntry) -> String {
    entry.file_name().into_string().unwrap()
}

fn main() {
    let Cli { path } = Cli::parse();

    if let Some(processed_path) = process_path(&path) {
        let result = get_dir_entries(&processed_path);
        match result {
            Ok(entries) => {
                entries
                    .iter()
                    .for_each(|e| println!("{}", get_file_name(e)));
            }
            Err(mes) => {
                eprintln!("Application error: {}", mes);
                process::exit(1);
            }
        }
    } else {
        println!("Failed to process path")
    }
}
