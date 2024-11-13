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
        let key = record.get(0).expect("Failed to get key");
        let value = record.get(1).expect("Failed to get value");
        map.insert(key.to_string(), value.to_string());
    }

    // Replace all occurrences in stdin of key to value.
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    for line in stdin.lock().lines() {
        let mut line = line.expect("Failed to read line");
        for (key, value) in &map {
            line = line.replace(key, value.trim());
        }
        writeln!(handle, "{}", line).expect("Failed to write line");
    }
}
