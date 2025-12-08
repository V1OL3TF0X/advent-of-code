pub struct Solution;
use crate::task_fns::SolveMode;

impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        file.lines()
            .map(get_bank)
            .map(|bank| find_number_from_n_max_digits(bank, 2))
            .sum::<u128>()
            .to_string()
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        file.lines()
            .map(get_bank)
            .map(|bank| find_number_from_n_max_digits(bank, 12))
            .sum::<u128>()
            .to_string()
    }
}

fn get_bank(line: &str) -> Vec<u32> {
    // SAFETY - valid input
    unsafe {
        line.chars()
            .map(|c| c.to_digit(10).unwrap_unchecked())
            .collect()
    }
}

fn find_number_from_n_max_digits(bank: Vec<u32>, n: usize) -> u128 {
    let mut result = 0;
    let mut start_index = 0;
    for i in 0..n {
        let end = bank.len() - n + i;

        // SAFETY - there will always be a max
        let mut max_index = start_index;
        for index in start_index..=end {
            if bank[index] > bank[max_index] {
                max_index = index;
            }
        }
        result = result * 10 + bank[max_index] as u128;
        start_index = max_index + 1;
    }
    result
}
