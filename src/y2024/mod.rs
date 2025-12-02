use once_cell::sync::Lazy;

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
        Box::new(day_14::Solution),
        Box::new(day_15::Solution),
        Box::new(day_16::Solution),
        Box::new(day_17::Solution),
        Box::new(day_18::Solution),
        Box::new(day_19::Solution),
        Box::new(day_20::Solution),
        Box::new(day_21::Solution),
        Box::new(day_22::Solution),
        Box::new(day_23::Solution),
        Box::new(day_24::Solution),
        Box::new(day_25::Solution),
    ]
});
