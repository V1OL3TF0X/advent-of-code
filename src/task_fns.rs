use clap::ValueEnum;

pub struct TaskFns {
    task_1: Box<dyn Fn(&str) -> String>,
    task_2: Box<dyn Fn(&str) -> String>,
}

impl TaskFns {
    pub fn run(&self, file: &str, mode: Task, day: usize) {
        let run_t = out(day, file);
        match mode {
            Task::Both => {
                run_t(1, &self.task_1);
                run_t(2, &self.task_2);
            }
            Task::First => run_t(1, &self.task_1),
            Task::Second => run_t(2, &self.task_2),
        }
    }

    pub fn new(task_1: Box<dyn Fn(&str) -> String>, task_2: Box<dyn Fn(&str) -> String>) -> Self {
        Self { task_1, task_2 }
    }
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Task {
    /// Run both tasks
    Both,
    /// Run only fist task of the day
    First,
    /// Run only second task of the day
    Second,
}

fn out(day: usize, input: &str) -> impl Fn(u8, &Box<dyn Fn(&str) -> String>) + '_ {
    move |task_no, task_fn| {
        println!(
            "Day {day}, task {task_no}: {}",
            crate::utils::measure_elapsed(|| task_fn(input))
        )
    }
}
