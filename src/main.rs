use core::{panic, str};
use std::{fmt::Display, str::FromStr};

use aoc::{
    task_fns::{SolveMode, Task, TaskFns},
    utils::{get_input, get_sample_input},
    y2023, y2024, y2025,
};
use chrono::{Datelike, Local};
use clap::Parser;

fn get_solutions_by_year(year: usize) -> (usize, fn(&usize) -> Result<&dyn TaskFns, String>) {
    match year {
        2023 => (y2023::MAX_DAY, y2023::get_solution_by_day),
        2024 => (y2025::MAX_DAY, y2024::get_solution_by_day),
        2025 => (y2025::MAX_DAY, y2025::get_solution_by_day),
        other => panic!("no solutions implemented for year {other}"),
    }
}

fn main() {
    let args = Args::parse();
    let (solved_task_number, get_task_fns) = get_solutions_by_year(args.year);
    if let AoCDay::One(from) = args.day_from {
        if from > solved_task_number {
            panic!(
                "Day from cannot be larger than the number of tasks done in year {} ({})",
                args.year, from
            );
        }
    }
    if let AoCDay::One(to) = args.day_to {
        if to > solved_task_number {
            panic!(
                "Day to cannot be larger than the number of tasks done in year {} ({})",
                args.year, to
            );
        }
    }
    let (get_file, input_type): (fn(usize, &str) -> String, SolveMode) = if args.sample {
        println!("Running with sample solution...");
        (get_sample_input, SolveMode::Sample)
    } else {
        println!("Running with real solution...");
        (get_input, SolveMode::Real)
    };
    let day_from = match args.day_from {
        AoCDay::All => 0,
        AoCDay::One(v) => v,
    };
    let day_to = match args.day_to {
        AoCDay::All => solved_task_number,
        AoCDay::One(v) => v,
    };
    (1..=solved_task_number)
        .skip(day_from - 1)
        .take(day_to - day_from + 1)
        .for_each(|day| {
            let file = get_file(args.year, &format!("day_{day}"));
            match get_task_fns(&day) {
                Ok(task_fn) => task_fn.run(&file, args.mode, day, input_type),
                Err(err) => println!("{err}"),
            }
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
