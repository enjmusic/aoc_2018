use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

fn get_repeat_info(box_id: &String) -> Result<(bool, bool)> {
    if !box_id.is_ascii() { return Err(From::from("Box ID not ASCII")); }
    let mut counts = (b'a'..=b'z').map(|c| (c as char, 0)).collect::<HashMap<char, usize>>();
    for c in box_id.to_lowercase().chars() {
        counts.get_mut(&c).map(|count| *count += 1);
    }
    Ok((counts.iter().any(|(_, &c)| c == 2), (counts.iter().any(|(_, &c)| c == 3))))
}

fn get_diff_indices(box_id_a: &String, box_id_b: &String) -> Result<HashSet<usize>> {
    if box_id_a.len() != box_id_b.len() { return Err(From::from("Box IDs have different lengths")); }
    let ret = box_id_a.chars().zip(box_id_b.chars()).enumerate()
        .filter_map(|(idx, (c_a, c_b))| if c_a != c_b { Some(idx) } else { None })
        .collect::<HashSet<usize>>();
    Ok(ret)
}

fn part1(box_ids: &Vec<String>) -> Result<()> {
    let (mut with_two, mut with_three) = (0, 0);
    for box_id in box_ids {
        let (has_two, has_three) = get_repeat_info(box_id)?;
        if has_two { with_two += 1; }
        if has_three { with_three += 1; }
    }
    Ok(println!("Checksum: {}", with_two * with_three))
}

fn part2(box_ids: &Vec<String>) -> Result<()> {
    for box_id_a in box_ids {
        for box_id_b in box_ids {
            let diff_indices = get_diff_indices(box_id_a, box_id_b)?;
            if diff_indices.len() == 1 {
                let common = box_id_a.char_indices()
                    .filter_map(|(idx, c)| if diff_indices.contains(&idx) { None } else { Some(c) })
                    .collect::<String>();
                return Ok(println!("Common characters: {}", common));
            }
        }
    }
    Err(From::from("No box IDs with 1 character difference found"))
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let f = File::open(opt.file)?;
    let reader = BufReader::new(f);
    let box_ids = reader.lines()
        .map(|l| l.map_err(|_| From::from("Couldn't read line")))
        .collect::<Result<Vec<String>>>()?;
    
    part1(&box_ids)?;
    part2(&box_ids)?;
    Ok(())
}