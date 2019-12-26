#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;
use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

#[derive(Debug)]
struct Claim {
    number: usize,
    top_left: (usize, usize),
    dimensions: (usize, usize),
}

impl Claim {
    fn parse(line: &String) -> Result<Claim> {
        lazy_static! {
            static ref CLAIM_REGEX: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        }
        if let Some(captures) = CLAIM_REGEX.captures(line) {
            return Ok(Claim{
                number: captures[1].parse::<usize>()?,
                top_left: (captures[2].parse::<usize>()?, captures[3].parse::<usize>()?),
                dimensions: (captures[4].parse::<usize>()?, captures[5].parse::<usize>()?)
            })
        }
        Err(From::from(format!("Invalid fabric claim line: {}", line)))
    }
}

fn part1(claims: &Vec<Claim>) -> Vec<Vec<usize>> {
    let mut grid = vec![vec![0usize; 1000]; 1000];
    for claim in claims {
        for col in claim.top_left.1..claim.top_left.1 + claim.dimensions.1 {
            for row in claim.top_left.0..claim.top_left.0 + claim.dimensions.0 {
                grid[col][row] += 1;
            }
        }
    }
    let count = grid.iter().fold(0, |acc, row| acc + row.iter().fold(0, |acc2, &cell| acc2 + (cell > 1) as usize));
    println!("Square inches inside more than one claim: {}", count);
    grid
}

fn part2(claims: &Vec<Claim>, grid: &Vec<Vec<usize>>) {
    for claim in claims {
        let no_overlap = (claim.top_left.1..claim.top_left.1 + claim.dimensions.1).all(|col| {
            (claim.top_left.0..claim.top_left.0 + claim.dimensions.0).all(|row| grid[col][row] == 1)
        });
        if no_overlap {
            return println!("Claim #{} overlaps with no other claims", claim.number);
        }
    }
    println!("All claims overlapped with at least one other claim")
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let f = File::open(opt.file)?;
    let reader = BufReader::new(f);
    let claims = reader.lines().map(|l| Claim::parse(&l?)).collect::<Result<Vec<Claim>>>()?;

    let grid = part1(&claims);
    part2(&claims, &grid);
    Ok(())
}