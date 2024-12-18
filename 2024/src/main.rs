use std::array::IntoIter;

use aoc_2024::utils::{get_input, get_sample_input, measure_elapsed};
use clap::{Parser, ValueEnum};

const AOC_PROBLEM_NO: usize = 17;

fn main() {
    let args = Args::parse();
    let task_fns = get_all_solution_fns();
    let get_file = if args.sample {
        println!("Running with sample solution...");
        get_sample_input
    } else {
        println!("Running with real solution...");
        get_input
    };
    task_fns
        .enumerate()
        .skip(args.day_from - 1)
        .take(args.day_to - args.day_from + 1)
        .for_each(|(i, t_fns)| {
            let t_no = i + 1;
            let file = get_file(&format!("day_{t_no}"));
            t_fns.run(&file, args.mode, t_no)
        });
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Task {
    /// Run both tasks
    Both,
    /// Run only fist task of the day
    First,
    /// Run only second task of the day
    Second,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// If provided, sample input will be passed to all tasks instead of real input
    #[clap(short = 's', long = "sample_input", default_value_t = false)]
    sample: bool,
    /// Smallest day number we want to run
    #[clap(short = 'f', long = "from", default_value_t = AOC_PROBLEM_NO, value_parser = day_in_range)]
    day_from: usize,
    /// Largest day number we want to run
    #[clap(short = 't', long = "to", default_value_t = AOC_PROBLEM_NO, value_parser = day_in_range)]
    day_to: usize,
    /// Which tasks to run for each day
    #[clap(value_enum, short, long, default_value_t = Task::Both)]
    mode: Task,
}

struct TaskFns {
    task_1: Box<dyn Fn(&str) -> String>,
    task_2: Box<dyn Fn(&str) -> String>,
}

fn out(day: usize, input: &str) -> impl Fn(u8, Box<dyn Fn(&str) -> String>) + '_ {
    move |task_no, task_fn| {
        println!(
            "Day {day}, task {task_no}: {}",
            measure_elapsed(|| task_fn(input))
        )
    }
}

impl TaskFns {
    fn run(self, file: &str, mode: Task, day: usize) {
        let run_t = out(day, file);
        match mode {
            Task::Both => {
                run_t(1, self.task_1);
                run_t(2, self.task_2);
            }
            Task::First => run_t(1, self.task_1),
            Task::Second => run_t(2, self.task_2),
        }
    }
}

fn get_all_solution_fns() -> IntoIter<TaskFns, AOC_PROBLEM_NO> {
    [
        TaskFns {
            task_1: Box::new(aoc_2024::day_1::task_1),
            task_2: Box::new(aoc_2024::day_1::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_2::task_1),
            task_2: Box::new(aoc_2024::day_2::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_3::task_1),
            task_2: Box::new(aoc_2024::day_3::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_4::task_1),
            task_2: Box::new(aoc_2024::day_4::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_5::task_1),
            task_2: Box::new(aoc_2024::day_5::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_6::task_1),
            task_2: Box::new(aoc_2024::day_6::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_7::task_1),
            task_2: Box::new(aoc_2024::day_7::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_8::task_1),
            task_2: Box::new(aoc_2024::day_8::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_9::task_1),
            task_2: Box::new(aoc_2024::day_9::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_10::task_1),
            task_2: Box::new(aoc_2024::day_10::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_11::task_1),
            task_2: Box::new(aoc_2024::day_11::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_12::task_1),
            task_2: Box::new(aoc_2024::day_12::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_13::task_1),
            task_2: Box::new(aoc_2024::day_13::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_14::task_1),
            task_2: Box::new(aoc_2024::day_14::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_15::task_1),
            task_2: Box::new(aoc_2024::day_15::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_16::task_1),
            task_2: Box::new(aoc_2024::day_16::task_2),
        },
        TaskFns {
            task_1: Box::new(aoc_2024::day_17::task_1),
            task_2: Box::new(aoc_2024::day_17::task_2),
        },
    ]
    .into_iter()
}

fn day_in_range(s: &str) -> Result<usize, String> {
    let day = s.parse().map_err(|_| format!("`{s}` isn't a day number"))?;
    if (1..=AOC_PROBLEM_NO).contains(&day) {
        Ok(day)
    } else {
        Err(format!("day not in range 1-{AOC_PROBLEM_NO}",))
    }
}
