use crate::utils::to_nums;

pub fn task_1(file: &str) -> String {
    file.lines()
        .map(to_nums)
        .map(get_next_in_seq)
        .sum::<isize>()
        .to_string()
}

pub fn task_2(file: &str) -> String {
    file.lines()
        .map(to_nums)
        .map(get_prev_in_seq)
        .sum::<isize>()
        .to_string()
}

fn get_next_in_seq(mut nums: Vec<isize>) -> isize {
    let mut last_in_seq = nums.len() - 1;
    let mut are_all_same;
    'count_diff: loop {
        nums[0] = nums[1] - nums[0];
        are_all_same = true;
        for i in 1..last_in_seq {
            nums[i] = nums[i + 1] - nums[i];
            if nums[i] != nums[i - 1] {
                are_all_same = false;
            }
        }
        last_in_seq -= 1;
        if are_all_same {
            break 'count_diff;
        }
    }
    nums.into_iter().skip(last_in_seq).sum()
}

fn get_prev_in_seq(mut nums: Vec<isize>) -> isize {
    let mut first_in_seq = 1;
    let last_ind = nums.len() - 1;
    let mut are_all_same;
    'count_diff: loop {
        nums[last_ind] -= nums[last_ind - 1];
        are_all_same = true;
        for i in (first_in_seq..last_ind).rev() {
            nums[i] -= nums[i - 1];
            if nums[i] != nums[i + 1] {
                are_all_same = false;
            }
        }
        first_in_seq += 1;
        if are_all_same {
            break 'count_diff;
        }
    }
    nums.into_iter()
        .take(first_in_seq)
        .enumerate()
        .map(|(i, n)| if i % 2 == 0 { n } else { -n })
        .sum()
}
