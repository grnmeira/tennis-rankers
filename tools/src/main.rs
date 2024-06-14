use std::fs::File;
use clap::{command, Parser };

/// A program that converts a CSV containing ranked players
/// into an SQL query. The SQL query can be used to import
/// the players into tennis-rankers.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// The CSV file.
    #[arg(short, long)]
    input: String,
    /// The generated SQL file.
    #[arg(short, long)]
    output: String,
}

#[derive(Default, Debug)]
struct Player {
    display_name: String,
    level: String,
    score: i32,
    gender: String,
    total_matches: u32,
    last_match_date: String,
    punishments: u32
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let mut csv_reader = csv::Reader::from_reader(input_file);
    let mut players = vec![];
    for record in csv_reader.records() {
        let record = record.unwrap();
        let p = Player {
            display_name: record.get(2).unwrap().to_string(),
            score: record.get(3).unwrap().parse::<i32>().unwrap(),
            gender: {
                if record.get(0).unwrap().contains("M") {
                    "M".to_string()
                } else if record.get(0).unwrap().contains("F") {
                    "F".to_string()
                } else {
                    "O".to_string()
                }
            },
            total_matches: record.get(7)
                .unwrap()
                .to_string()
                .replace("*", "")
                .parse::<u32>()
                .unwrap(),
            ..Default::default()
        };
        players.push(p);
    }
    for p in players{
        println!("{p:?}");
    }
}