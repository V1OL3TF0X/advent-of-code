use itertools::Itertools;

pub struct Solution;

impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        file.lines()
            .map(get_bank)
            .map(|bank| find_number_from_n_max_digits(bank, 2))
            .sum::<u128>()
            .to_string()
    }

    fn task_2(&self, file: &str) -> String {
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
    println!("searching in {bank:?} (len {}):", bank.len());
    for i in 0..n {
        let end = bank.len() - n + i;
        println!("   looking for {i}-th digit, starting from {start_index} ending at {end}");

        // SAFETY - there will always be a max
        let mut max_index = start_index;
        for index in start_index..=end {
            if bank[index] > bank[max_index] {
                max_index = index;
            }
        }
        println!("    found {} at index {}", bank[max_index], max_index);
        result = result * 10 + bank[max_index] as u128;
        start_index = max_index + 1;
    }
    result
}
