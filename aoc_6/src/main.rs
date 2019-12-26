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

#[derive(Clone)]
struct Tile {
    total_distance: usize,
    status: VoronoiStatus,
}

#[derive(Clone)]
enum VoronoiStatus {
    Unknown,
    Conflict(usize),
    ClosestTo(usize, usize),
}

struct VoronoiGrid {
    grid: Vec<Vec<Tile>>
}

impl VoronoiGrid {
    fn parse(reader: BufReader<File>) -> Result<VoronoiGrid> {
        let points = reader.lines().map(|l| {
            let line = l?;
            let split = line.splitn(2, ", ").collect::<Vec<&str>>();
            if split.len() != 2 { return Err(From::from("Invalid point")) }
            Ok((split[0].parse::<i32>()?, split[1].parse::<i32>()?))
        }).collect::<Result<Vec<(i32, i32)>>>()?;

        // Calculate bounds of grid
        let mut top_left = (std::i32::MAX, std::i32::MAX);
        let mut bottom_right = (0i32, 0i32);
        for point in &points {
            let p = (point.0 as i32, point.1 as i32);
            if p.0 < top_left.0 { top_left.0 = p.0 }
            if p.1 < top_left.1 { top_left.1 = p.1 }
            if p.0 > bottom_right.0 { bottom_right.0 = p.0 }
            if p.1 > bottom_right.1 { bottom_right.1 = p.1 }
        }
        // Expand bounds by 1 in all directions so we can detect infinite areas
        top_left = (top_left.0 - 1, top_left.1 - 1);
        bottom_right = (bottom_right.0 + 1, bottom_right.1 + 1);

        // Calculate status of each point in the grid
        let (width, height) = (1 + bottom_right.0 - top_left.0, 1 + bottom_right.1 - top_left.1);
        let grid = (0..height).map(|row| (0..width).map(|col| {
            let (x, y) = (col + top_left.0, row + top_left.1);
            let (mut curr_status, mut total_dist) = (VoronoiStatus::Unknown, 0);
            for (idx, point) in points.iter().enumerate() {
                let dist: usize = ((point.0 - x).abs() + (point.1 - y).abs()) as usize;
                total_dist += dist;
                curr_status = match curr_status {
                    VoronoiStatus::Unknown => VoronoiStatus::ClosestTo(idx, dist),
                    VoronoiStatus::Conflict(d) => {
                        if dist < d { VoronoiStatus::ClosestTo(idx, dist) } else { curr_status }
                    },
                    VoronoiStatus::ClosestTo(_, d) => {
                        match dist.cmp(&d) {
                            std::cmp::Ordering::Less => VoronoiStatus::ClosestTo(idx, dist),
                            std::cmp::Ordering::Equal => VoronoiStatus::Conflict(d),
                            std::cmp::Ordering::Greater => curr_status
                        }
                    },
                }
            }
            Tile{ total_distance: total_dist, status: curr_status }
        }).collect()).collect::<Vec<Vec<Tile>>>();
        Ok(VoronoiGrid { grid: grid })
    }
}

fn part1(grid: &VoronoiGrid) {
    let mut infinite_areas: HashSet<usize> = HashSet::new();
    let dimensions = (grid.grid[0].len(), grid.grid.len());
    for row in 0..dimensions.1 {
        for col in 0..dimensions.0 {
            if row == 0 || row == dimensions.1 - 1 || col == 0 || col == dimensions.0 - 1 {
                if let VoronoiStatus::ClosestTo(idx, _) = grid.grid[row][col].status {
                    infinite_areas.insert(idx);
                }
            }
        }
    }

    let mut areas: HashMap<usize, usize> = HashMap::new();
    for row in &grid.grid {
        for cell in row {
            if let VoronoiStatus::ClosestTo(idx, _) = cell.status {
                if !infinite_areas.contains(&idx) {
                    let entry = areas.entry(idx).or_insert(0);
                    *entry += 1;
                }
            }
        }
    }

    let largest_area = areas.values().max().unwrap_or(&0);
    println!("Largest non-infinite area: {}", largest_area);
}

fn part2(grid: &VoronoiGrid) {
    let mut under_threshold_count = 0;
    for row in &grid.grid {
        for cell in row {
            if cell.total_distance < 10000 { under_threshold_count += 1; }
        }
    }
    println!("Region containing all locations with total distance < 10000 is {}", under_threshold_count);
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let f = File::open(opt.file)?;
    let reader = BufReader::new(f);
    let grid = VoronoiGrid::parse(reader)?;

    part1(&grid);
    part2(&grid);
    Ok(())
}