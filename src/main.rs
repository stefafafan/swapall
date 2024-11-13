use std::io::{BufRead, Write};

use clap::Parser;
use csv::ReaderBuilder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "The filepath to the CSV file.", required = true)]
    csv_path: String,
}

fn main() {
    let args = Args::parse();

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(&args.csv_path)
        .expect("Failed to create CSV reader");

    let mut map = std::collections::HashMap::new();

    for result in csv_reader.records() {
        let record = result.expect("Failed to parse CSV record");
        if record.len() != 2 {
            eprintln!("Expected 2 columns, got {}", record.len());
            std::process::exit(1);
        }
        let from = record.get(0).expect("Failed to get the first value");
        let to = record.get(1).expect("Failed to get the second value");
        map.insert(from.to_string(), to.to_string());
    }

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    for line in stdin.lock().lines() {
        let mut line = line.expect("Failed to read line");
        for (from, to) in &map {
            line = line.replace(from, to.trim());
        }
        writeln!(handle, "{}", line).expect("Failed to write line");
    }
}
