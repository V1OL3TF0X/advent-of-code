use clap::ValueEnum;

pub trait TaskFns {
    fn task_1(&self, file: &str) -> String;
    fn task_2(&self, file: &str) -> String;
    fn run(&self, file: &str, mode: Task, day: usize) {
        let run_t_1 = || {
            println!(
                "Day {day}, task 1: {}",
                crate::utils::measure_elapsed(|| self.task_1(file))
            );
        };
        let run_t_2 = || {
            println!(
                "Day {day}, task 2: {}",
                crate::utils::measure_elapsed(|| self.task_2(file))
            );
        };
        match mode {
            Task::Both => {
                run_t_1();
                run_t_2();
            }
            Task::First => run_t_1(),
            Task::Second => run_t_2(),
        }
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
