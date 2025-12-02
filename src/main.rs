use core::{panic, str};
use std::{fmt::Display, str::FromStr};

use aoc::{
    task_fns::{Task, TasksDefinition},
    utils::{get_input, get_sample_input},
    y2023, y2024, y2025,
};
use chrono::{Datelike, Local};
use clap::Parser;
use once_cell::sync::Lazy;

const SOLUTIONS_BY_YEAR: Lazy<[TasksDefinition; 3]> =
    Lazy::new(|| -> [TasksDefinition; 3] { [y2023::TASKS, y2024::TASKS, y2025::TASKS] });

fn main() {
    let args = Args::parse();
    let task_fns = &SOLUTIONS_BY_YEAR[args.year - 2023];
    if let AoCDay::One(from) = args.day_from {
        if from > task_fns.len() {
            panic!(
                "Day from cannot be larger than the number of tasks done in year {} ({})",
                args.year, from
            );
        }
    }
    if let AoCDay::One(to) = args.day_to {
        if to > task_fns.len() {
            panic!(
                "Day to cannot be larger than the number of tasks done in year {} ({})",
                args.year, to
            );
        }
    }
    let get_file = if args.sample {
        println!("Running with sample solution...");
        get_sample_input
    } else {
        println!("Running with real solution...");
        get_input
    };
    let day_from = match args.day_from {
        AoCDay::All => 0,
        AoCDay::One(v) => v,
    };
    let day_to = match args.day_to {
        AoCDay::All => task_fns.len(),
        AoCDay::One(v) => v,
    };
    task_fns
        .iter()
        .enumerate()
        .skip(day_from - 1)
        .take(day_to - day_from + 1)
        .for_each(|(i, t_fns)| {
            let t_no = i + 1;
            let file = get_file(args.year, &format!("day_{t_no}"));
            t_fns.run(&file, args.mode, t_no)
        });
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// If provided, sample input will be passed to all tasks instead of real input
    #[clap(short = 's', long = "sample_input", default_value_t = false)]
    sample: bool,
    /// Smallest day number we want to run
    #[clap(short = 'f', long = "from", default_value_t = current_aoc_day(), value_parser = day_in_range)]
    day_from: AoCDay,
    /// Largest day number we want to run
    #[clap(short = 't', long = "to", default_value_t = current_aoc_day(), value_parser = day_in_range)]
    day_to: AoCDay,
    /// year_of_aoc
    #[clap(short = 'y', long = "year", default_value_t = current_aoc_year())]
    year: usize,
    /// Which tasks to run for each day
    #[clap(value_enum, short, long, default_value_t = Task::Both)]
    mode: Task,
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum AoCDay {
    All,
    One(usize),
}

impl Display for AoCDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AoCDay::All => "all".to_string(),
                AoCDay::One(v) => v.to_string(),
            }
        )
    }
}

impl FromStr for AoCDay {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "all" {
            return Ok(AoCDay::All);
        }
        s.parse()
            .map(AoCDay::One)
            .map_err(|_| format!("couldn't parse AoC day into usize: {s}"))
    }
}

fn current_aoc_year() -> usize {
    let now = Local::now();
    if now.month() == 12 {
        return now.year() as usize;
    }
    now.year() as usize - 1
}
fn current_aoc_day() -> AoCDay {
    let now = Local::now();
    if now.month() == 12 {
        return AoCDay::One(now.day() as usize);
    }
    AoCDay::All
}

fn day_in_range(s: &str) -> Result<AoCDay, String> {
    if s == "all" {
        return Ok(AoCDay::All);
    }
    let day = s.parse().map_err(|_| format!("`{s}` isn't a day number"))?;
    if day != 0 && day < 25 {
        Ok(AoCDay::One(day))
    } else {
        Err("day not in range 1-25".into())
    }
}
