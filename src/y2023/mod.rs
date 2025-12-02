use once_cell::sync::Lazy;

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
pub const TASKS: crate::task_fns::TasksDefinition = Lazy::new(|| {
    vec![
        Box::new(day_1::Solution),
        Box::new(day_2::Solution),
        Box::new(day_3::Solution),
        Box::new(day_4::Solution),
        Box::new(day_5::Solution),
        Box::new(day_6::Solution),
        Box::new(day_7::Solution),
        Box::new(day_8::Solution),
        Box::new(day_9::Solution),
        Box::new(day_10::Solution),
        Box::new(day_11::Solution),
        Box::new(day_12::Solution),
        Box::new(day_13::Solution),
    ]
});
