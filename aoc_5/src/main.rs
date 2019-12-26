use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

fn react_once(polymer: &String) -> String {
    let mut out: Vec<char> = vec![];
    polymer.chars().for_each(|c| {
        if let Some(prev) = out.last() {
            if prev.eq_ignore_ascii_case(&c) && prev.is_ascii_lowercase() != c.is_ascii_lowercase() {
                out.pop();
            } else {
                out.push(c);
            }
        } else {
            out.push(c)
        }
    });
    out.iter().collect()
}

fn fully_react(polymer: &String, without: Option<char>) -> String {
    let mut last = polymer.chars()
        .filter(|&c| without.is_none() || c.to_ascii_lowercase() != without.unwrap())
        .collect::<String>();
    let mut curr = react_once(&last);
    while curr.len() != 0 && curr != last {
        last = curr;
        curr = react_once(&last);
    }
    curr
}

fn part1(polymer: &String) {
    println!("Units remaining after polymer was fully reacted: {}", fully_react(polymer, None).len());
}

fn part2(polymer: &String) {
    let min_reacted_length = (b'a'..=b'z').fold(std::usize::MAX, |acc, c| {
        let fully_reacted = fully_react(polymer, Some(c as char));
        if fully_reacted.len() < acc { fully_reacted.len() } else { acc }
    });
    println!("Shortest possible polymer with 1 unit type removal: {}", min_reacted_length);
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let f = File::open(opt.file)?;
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    part1(&contents);
    part2(&contents);
    Ok(())
}