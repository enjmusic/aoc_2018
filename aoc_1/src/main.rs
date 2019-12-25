use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::collections::HashSet;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

fn part1(frequency_changes: &Vec<i64>) {
    println!("Resulting frequency: {}", frequency_changes.iter().map(|&c| c).sum::<i64>());
}

fn part2(frequency_changes: &Vec<i64>) {
    let mut seen = HashSet::new();
    let mut curr = 0;
    for &change in frequency_changes.iter().cycle() {
        if seen.contains(&curr) { break }
        seen.insert(curr);
        curr += change;
    }
    println!("First frequency reached twice: {}", curr);
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let f = File::open(opt.file)?;
    let reader = BufReader::new(f);
    let frequency_changes = reader.lines()
        .map(|l| l?.parse::<i64>().map_err(|_| From::from("Invalid input")))
        .collect::<Result<Vec<i64>>>()?;
    
    part1(&frequency_changes);
    part2(&frequency_changes);
    Ok(())
}
