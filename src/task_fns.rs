use clap::ValueEnum;

pub trait TaskFns {
    fn task_1(&self, file: &str, mode: SolveMode) -> String;
    fn task_2(&self, file: &str, mode: SolveMode) -> String;
    fn run(&self, file: &str, mode: Task, day: usize, solve: SolveMode) {
        println!("Day {day}:");
        let run_t_1 = || {
            let (dur, res) = crate::utils::measure_elapsed(|| self.task_1(file, solve));
            println!("    task 1: {res} ({dur:.2?})",);
        };
        let run_t_2 = || {
            let (dur, res) = crate::utils::measure_elapsed(|| self.task_2(file, solve));
            println!("    task 2: {res} ({dur:.2?})",);
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

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum SolveMode {
    /// sample input
    Sample,
    /// real input
    Real,
}
