use chrono::NaiveDate;
use clap::{command, Parser};
use std::fs::File;
use uuid::Uuid;

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
    id: Uuid,
    display_name: String,
    level: String,
    score: i32,
    gender: String,
    total_matches: u32,
    last_match_date: String,
    punishments: u32,
}

fn main() {
    let args = Args::parse();

    let input_file = File::open(args.input).unwrap();
    let mut csv_reader = csv::Reader::from_reader(input_file);
    let mut players = vec![];
    for record in csv_reader.records() {
        let record = record.unwrap();
        let pos = record
            .position()
            .expect("Unable to get position for CSV record");
        let last_match_date = record.get(4).expect(&format!(
            "Unable to get date for last match at position {pos:?}"
        ));
        let last_match_date = NaiveDate::parse_from_str(last_match_date, "%d/%m/%Y").expect(
            &format!("Unable to parse date from {pos:?} {last_match_date}"),
        );
        let last_match_date = last_match_date.format("%Y-%m-%d").to_string();
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
            total_matches: record
                .get(7)
                .unwrap()
                .to_string()
                .replace("*", "")
                .parse::<u32>()
                .unwrap_or_default(),
            punishments: record
                .get(7)
                .unwrap()
                .to_string()
                .chars()
                .filter(|&c| c == '*')
                .count() as u32,
            id: Uuid::new_v4(),
            level: match record.get(0).unwrap().split("/").nth(0).unwrap() {
                "NB" | "B" => "intermediate",
                "NC" | "C" => "intermediate+",
                "ND" | "D" => "advanced",
                _ => "beginner",
            }
            .to_string(),
            last_match_date,
            ..Default::default()
        };
        players.push(p);
    }
    for p in players {
        println!(
            "INSERT INTO players(id, display_name, player_level, gender, _legacy_last_match_date, _legacy_total_matches, _legacy_total_punishments) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}');",
            p.id, p.display_name, p.level, p.gender, p.last_match_date, p.total_matches, p.punishments);
        println!(
            "INSERT INTO score_ledger(player_id, reason, score, comment) VALUES ('{}', '{}', {}, '{}');",
            p.id, "other", p.score, "score from legacy system"
        );
    }
}
