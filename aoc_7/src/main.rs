use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::collections::{BTreeMap, HashMap, HashSet};
use structopt::StructOpt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

struct Dependencies {
    steps_to_unlock: BTreeMap<char, usize>,
    unlocks: HashMap<char, Vec<char>>
}

impl Dependencies {
    fn parse(reader: BufReader<File>) -> Result<Dependencies> {
        let mut ret = Dependencies {
            steps_to_unlock: BTreeMap::new(),
            unlocks: HashMap::new()
        };

        for line in reader.lines() {
            let l = line?;
            let trimmed = l.trim_start_matches("Step ").trim_end_matches(" can begin.");
            let split = trimmed.splitn(2, " must be finished before step ").collect::<Vec<&str>>();
            if split.len() != 2 || !split.iter().all(|c| c.len() == 1) {
                return Err(From::from(format!("Invalid line: {}", l)));
            }
            let (req, unlock) = (split[0].chars().next().unwrap(), split[1].chars().next().unwrap());

            ret.steps_to_unlock.entry(req).or_insert(0);
            *ret.steps_to_unlock.entry(unlock).or_insert(0) += 1;
            ret.unlocks.entry(req).or_insert(vec![]).push(unlock);
        }
        Ok(ret)
    }
}

fn do_steps_with_workers(deps: &Dependencies, num_workers: usize, base_step_time: u32) -> (String, u32) {
    let mut time_spent = 0;
    let mut worker_time_until_free: Vec<u32> = vec![0; num_workers];
    let mut in_progress: Vec<Option<char>> = vec![None; num_workers];
    let mut unlocked: HashSet<char> = HashSet::new();
    let mut sequence: Vec<char> = vec![];
    let mut steps_left = deps.steps_to_unlock.clone();

    while unlocked.len() < deps.steps_to_unlock.len() {
        let min_time_until_worker_free = *worker_time_until_free.iter().filter(|&&t| t != 0).min().unwrap_or(&0);
        time_spent += min_time_until_worker_free;

        for (idx, until_free) in (&mut worker_time_until_free).iter_mut().enumerate() {
            if *until_free <= min_time_until_worker_free {
                *until_free = 0;
                if let Some(finished) = in_progress[idx].take() {
                    for unlock in deps.unlocks.get(&finished).unwrap_or(&vec![]) {
                        steps_left.get_mut(unlock).map(|to_dec| *to_dec -= 1);
                    }
                }
            } else {
                *until_free -= min_time_until_worker_free;
            }
        }

        for (idx, until_free) in (&mut worker_time_until_free).iter_mut().enumerate() {
            if *until_free > 0 { continue }
            let next = steps_left.iter().skip_while(|(c, &r)| unlocked.contains(c) || r > 0).next();
            if let Some((&c, _)) = next {
                sequence.push(c);
                unlocked.insert(c);
                *until_free = base_step_time + 1 + ((c as u8) - ('A' as u8)) as u32;
                in_progress[idx] = Some(c);
            }
        }
    }

    (sequence.into_iter().collect::<String>(), time_spent + *worker_time_until_free.iter().max().unwrap())
}

fn part1(deps: &Dependencies) {
    println!("Step order: {}", do_steps_with_workers(deps, 1, 0).0);
}

fn part2(deps: &Dependencies) {
    println!("Total time spent on tasks with 5 workers: {}", do_steps_with_workers(deps, 5, 60).1);
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let f = File::open(opt.file)?;
    let reader = BufReader::new(f);
    let deps = Dependencies::parse(reader)?;

    part1(&deps);
    part2(&deps);
    Ok(())
}