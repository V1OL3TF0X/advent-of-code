use once_cell::sync::Lazy;

use crate::task_fns::TaskFns;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
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
        TaskFns::new(Box::new(day_14::task_1), Box::new(day_14::task_2)),
        TaskFns::new(Box::new(day_15::task_1), Box::new(day_15::task_2)),
        TaskFns::new(Box::new(day_16::task_1), Box::new(day_16::task_2)),
        TaskFns::new(Box::new(day_17::task_1), Box::new(day_17::task_2)),
        TaskFns::new(Box::new(day_18::task_1), Box::new(day_18::task_2)),
        TaskFns::new(Box::new(day_19::task_1), Box::new(day_19::task_2)),
        TaskFns::new(Box::new(day_20::task_1), Box::new(day_20::task_2)),
        TaskFns::new(Box::new(day_21::task_1), Box::new(day_21::task_2)),
        TaskFns::new(Box::new(day_22::task_1), Box::new(day_22::task_2)),
        TaskFns::new(Box::new(day_23::task_1), Box::new(day_23::task_2)),
        TaskFns::new(Box::new(day_24::task_1), Box::new(day_24::task_2)),
        TaskFns::new(Box::new(day_25::task_1), Box::new(day_25::task_2)),
    ]
});
