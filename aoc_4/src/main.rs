use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::collections::HashMap;
use structopt::StructOpt;
use chrono::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "f", parse(from_os_str))]
    file: PathBuf,
}

#[derive(Debug)]
enum GuardActionType {
    FallAsleep,
    WakeUp,
    BeginShift(usize)
}

#[derive(Debug)]
struct GuardAction {
    time: DateTime<Utc>,
    action_type: GuardActionType
}

impl GuardAction {
    fn parse(input: &String) -> Result<GuardAction> {
        let split_time_action = input.splitn(2, "] ").collect::<Vec<&str>>();
        if split_time_action.len() != 2 { return Err(From::from("Invalid format")); }
        let time = Utc.datetime_from_str(split_time_action[0].trim_start_matches("["), "%Y-%m-%d %H:%M")?;
        let action_type = match split_time_action[1] {
            "falls asleep" => GuardActionType::FallAsleep,
            "wakes up" => GuardActionType::WakeUp,
            raw => {
                let guard_number = raw.trim_start_matches("Guard #").trim_end_matches(" begins shift").parse::<usize>()?;
                GuardActionType::BeginShift(guard_number)
            },
        };
        Ok( GuardAction{ time: time, action_type: action_type } )
    }
}

fn get_guard_sleep_intervals(actions: &Vec<GuardAction>) -> HashMap<usize, Vec<(u32, u32)>> {
    let mut guard_sleep: HashMap<usize, Vec<(u32, u32)>> = HashMap::new();
    let mut curr_guard: Option<usize> = None;
    let mut sleep_start: Option<u32> = None;
    for action in actions {
        match action.action_type {
            GuardActionType::BeginShift(guard_id) => curr_guard = Some(guard_id),
            GuardActionType::FallAsleep => sleep_start = Some(action.time.minute()),
            GuardActionType::WakeUp => {
                let entry = guard_sleep.entry(curr_guard.unwrap_or(0)).or_insert(vec![]);
                entry.push((sleep_start.unwrap_or(0), action.time.minute()));
                sleep_start = None;
            }
        }
    }
    guard_sleep
}

fn get_most_commonly_slept_minute(naps: &Vec<(u32, u32)>) -> (usize, u32) {
    let mut minute_sleep_counts = vec![0; 60];
    for nap in naps {
        for minute in nap.0..nap.1 {
            minute_sleep_counts[minute as usize] += 1;
        }
    }

    minute_sleep_counts.iter().enumerate().fold((0, 0u32), |curr_best, (min, &freq)| {
        if freq > curr_best.1 { (min, freq) } else { curr_best }
    })
}

fn part1(sleep_intervals: &HashMap<usize, Vec<(u32, u32)>>) {
    let (most_sleep_guard_id, _) = sleep_intervals.iter().fold((0, 0), |curr_best, (&id, naps)| {
        let sleep_total = naps.iter().fold(0, |acc, (start, end)| acc + (end - start));
        if sleep_total > curr_best.1 { (id, sleep_total) } else { curr_best }
    });

    let (most_often_slept_minute, _) = get_most_commonly_slept_minute(&sleep_intervals[&most_sleep_guard_id]);

    println!(
        "Guard {} slept the most, with minute {} being the most frequent (product = {})",
        most_sleep_guard_id, most_often_slept_minute, most_sleep_guard_id * most_often_slept_minute
    );
}

fn part2(sleep_intervals: &HashMap<usize, Vec<(u32, u32)>>) {
    let (guard_id, minute, _) = sleep_intervals.iter().fold((0, 0, 0), |curr_best, (&id, naps)| {
        let (minute, count) = get_most_commonly_slept_minute(naps);
        if count > curr_best.2 { (id, minute, count) } else { curr_best }
    });

    println!(
        "Guard {} spent minute {} asleep more than any other guard or minute (product = {})",
        guard_id, minute, guard_id * minute
    );
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let f = File::open(opt.file)?;
    let reader = BufReader::new(f);
    let mut guard_actions = reader.lines().map(|l| GuardAction::parse(&l?)).collect::<Result<Vec<GuardAction>>>()?;
    guard_actions.sort_by(|ga1, ga2| ga1.time.cmp(&ga2.time));
    let sleep_intervals = get_guard_sleep_intervals(&guard_actions);

    part1(&sleep_intervals);
    part2(&sleep_intervals);
    Ok(())
}