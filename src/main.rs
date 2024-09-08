use clap::Parser;
use kbt::process_path;

#[derive(Parser)]
struct Cli {
    path: String,
}

fn main() {
    let Cli { path } = Cli::parse();

    if let Some(processed_path) = process_path(&path) {
        println!("Processed path: {:?}", processed_path)
    } else {
        println!("Failed to process path")
    }
}
