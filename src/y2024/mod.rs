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

pub const MAX_DAY: usize = 25;
pub fn get_solution_by_day(day: &usize) -> Result<&dyn TaskFns, String> {
    match day {
        1 => Ok(&day_1::Solution),
        2 => Ok(&day_2::Solution),
        3 => Ok(&day_3::Solution),
        4 => Ok(&day_4::Solution),
        5 => Ok(&day_5::Solution),
        6 => Ok(&day_6::Solution),
        7 => Ok(&day_7::Solution),
        8 => Ok(&day_8::Solution),
        9 => Ok(&day_9::Solution),
        10 => Ok(&day_10::Solution),
        11 => Ok(&day_11::Solution),
        12 => Ok(&day_12::Solution),
        13 => Ok(&day_13::Solution),
        14 => Ok(&day_14::Solution),
        15 => Ok(&day_15::Solution),
        16 => Ok(&day_16::Solution),
        17 => Ok(&day_17::Solution),
        18 => Ok(&day_18::Solution),
        19 => Ok(&day_19::Solution),
        20 => Ok(&day_20::Solution),
        21 => Ok(&day_21::Solution),
        22 => Ok(&day_22::Solution),
        23 => Ok(&day_23::Solution),
        24 => Ok(&day_24::Solution),
        25 => Ok(&day_25::Solution),
        other => Err(format!(
            "year 2023 has no solution implemented for day {other}"
        )),
    }
}
