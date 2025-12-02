use once_cell::sync::Lazy;

use crate::task_fns::TaskFns;

mod day_1;
//pub mod day_10;
// pub mod day_11;
// pub mod day_12;
// pub mod day_13;
// pub mod day_14;
// pub mod day_15;
// pub mod day_16;
// pub mod day_17;
// pub mod day_18;
// pub mod day_19;
mod day_2;
// pub mod day_20;
// pub mod day_21;
// pub mod day_22;
// pub mod day_3;
// pub mod day_4;
// pub mod day_5;
// pub mod day_6;
// pub mod day_7;
// pub mod day_8;
// pub mod day_9;
pub const TASKS: Lazy<Vec<TaskFns>> = Lazy::new(|| {
    vec![
        TaskFns::new(Box::new(day_1::task_1), Box::new(day_1::task_2)),
        TaskFns::new(Box::new(day_2::task_1), Box::new(day_2::task_2)),
        //        TaskFns::new(Box::new(day_3::task_1), Box::new(day_3::task_2)),
        //        TaskFns::new(Box::new(day_4::task_1), Box::new(day_4::task_2)),
        //        TaskFns::new(Box::new(day_5::task_1), Box::new(day_5::task_2)),
        //        TaskFns::new(Box::new(day_6::task_1), Box::new(day_6::task_2)),
        //        TaskFns::new(Box::new(day_7::task_1), Box::new(day_7::task_2)),
        //        TaskFns::new(Box::new(day_8::task_1), Box::new(day_8::task_2)),
        //        TaskFns::new(Box::new(day_9::task_1), Box::new(day_9::task_2)),
        //        TaskFns::new(Box::new(day_10::task_1), Box::new(day_10::task_2)),
        //        TaskFns::new(Box::new(day_11::task_1), Box::new(day_11::task_2)),
        //        TaskFns::new(Box::new(day_12::task_1), Box::new(day_12::task_2)),
        //        TaskFns::new(Box::new(day_13::task_1), Box::new(day_13::task_2)),
        //        TaskFns::new(Box::new(day_14::task_1), Box::new(day_14::task_2)),
        //        TaskFns::new(Box::new(day_15::task_1), Box::new(day_15::task_2)),
        //        TaskFns::new(Box::new(day_16::task_1), Box::new(day_16::task_2)),
        //        TaskFns::new(Box::new(day_17::task_1), Box::new(day_17::task_2)),
        //        TaskFns::new(Box::new(day_18::task_1), Box::new(day_18::task_2)),
        //        TaskFns::new(Box::new(day_19::task_1), Box::new(day_19::task_2)),
        //        TaskFns::new(Box::new(day_20::task_1), Box::new(day_20::task_2)),
        //        TaskFns::new(Box::new(day_21::task_1), Box::new(day_21::task_2)),
        //        TaskFns::new(Box::new(day_22::task_1), Box::new(day_22::task_2)),
        //        TaskFns::new(Box::new(day_23::task_1), Box::new(day_23::task_2)),
        //        TaskFns::new(Box::new(day_24::task_1), Box::new(day_24::task_2)),
        //        TaskFns::new(Box::new(day_25::task_1), Box::new(day_25::task_2)),
    ]
});
