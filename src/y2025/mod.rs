use crate::task_fns::TaskFns;

mod day_1;
// mod day_10;
// mod day_11;
// mod day_12;
mod day_2;
mod day_3;
mod day_4;
// mod day_5;
// mod day_6;
// mod day_7;
// mod day_8;
// mod day_9;
pub const MAX_DAY: usize = 4;
pub fn get_solution_by_day(day: &usize) -> Result<&dyn TaskFns, String> {
    match day {
        1 => Ok(&day_1::Solution),
        2 => Ok(&day_2::Solution),
        3 => Ok(&day_3::Solution),
        4 => Ok(&day_4::Solution),
        //  5 => Ok(Box::new(day_5::Solution)),
        //  6 => Ok(Box::new(day_6::Solution)),
        //  7 => Ok(Box::new(day_7::Solution)),
        //  8 => Ok(Box::new(day_8::Solution)),
        //  9 => Ok(Box::new(day_9::Solution)),
        //  10 => Ok(Box::new(day_10::Solution)),
        //  11 => Ok(Box::new(day_11::Solution)),
        //  12 => Ok(Box::new(day_12::Solution)),
        other => Err(format!(
            "year 2023 has no solution implemented for day {other}"
        )),
    }
}
