mod cli;
mod walker;

use cli::Cli;
use clap::Parser;
use walker::collect_files;

fn main() {
    let args = Cli::parse();

    let files = collect_files(&args.path);

    println!("Found {} files:", files.len());
    for file in files {
        println!("{}", file);
    }
}