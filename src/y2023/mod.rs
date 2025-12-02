use once_cell::sync::Lazy;

use crate::task_fns::TaskFns;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
pub const TASKS: Lazy<Vec<TaskFns>> = Lazy::new(|| {
    vec![
        TaskFns::new(Box::new(day_1::task_1), Box::new(day_1::task_2)),
        TaskFns::new(Box::new(day_2::task_1), Box::new(day_2::task_2)),
        TaskFns::new(Box::new(day_3::task_1), Box::new(day_3::task_2)),
        TaskFns::new(Box::new(day_4::task_1), Box::new(day_4::task_2)),
        TaskFns::new(Box::new(day_5::task_1), Box::new(day_5::task_2)),
        TaskFns::new(Box::new(day_6::task_1), Box::new(day_6::task_2)),
        TaskFns::new(Box::new(day_7::task_1), Box::new(day_7::task_2)),
        TaskFns::new(Box::new(day_8::task_1), Box::new(day_8::task_2)),
        TaskFns::new(Box::new(day_9::task_1), Box::new(day_9::task_2)),
        TaskFns::new(Box::new(day_10::task_1), Box::new(day_10::task_2)),
        TaskFns::new(Box::new(day_11::task_1), Box::new(day_11::task_2)),
        TaskFns::new(Box::new(day_12::task_1), Box::new(day_12::task_2)),
        TaskFns::new(Box::new(day_13::task_1), Box::new(day_13::task_2)),
    ]
});
