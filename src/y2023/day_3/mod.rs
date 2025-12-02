use crate::task_fns::TaskFns;

mod task_1;
mod task_2;
mod utils;

pub struct Solution;
impl TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        task_1::task_1(file)
    }

    fn task_2(&self, file: &str) -> String {
        task_2::task_2(file)
    }
}
