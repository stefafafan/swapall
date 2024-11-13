use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "The filepath to the CSV file.", required = true)]
    csv_path: String,
}

fn main() {
    let args = Args::parse();

    let file = std::fs::File::open(&args.csv_path).expect("Failed to open file");
    let reader = std::io::BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(reader);

    for result in csv_reader.records() {
        let record = result.expect("Failed to read record");
        println!("{:?}", record);
    }
}
