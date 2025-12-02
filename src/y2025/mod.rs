use once_cell::sync::Lazy;

mod day_1;
//pub mod day_10;
// pub mod day_11;
// pub mod day_12;
mod day_2;
// pub mod day_3;
// pub mod day_4;
// pub mod day_5;
// pub mod day_6;
// pub mod day_7;
// pub mod day_8;
// pub mod day_9;
pub const TASKS: crate::task_fns::TasksDefinition = Lazy::new(|| {
    vec![
        Box::new(day_1::Solution),
        Box::new(day_2::Solution),
        //        Box::new(day_3::Solution),
        //        Box::new(day_4::Solution),
        //        Box::new(day_5::Solution),
        //        Box::new(day_6::Solution),
        //        Box::new(day_7::Solution),
        //        Box::new(day_8::Solution),
        //        Box::new(day_9::Solution),
        //        Box::new(day_10::Solution),
        //        Box::new(day_11::Solution),
        //        Box::new(day_12::Solution),
    ]
});
