pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        file.lines()
            .filter(|l| {
                let nums: Vec<_> = l.split_whitespace().flat_map(str::parse::<u32>).collect();
                is_safe(&nums)
            })
            .count()
            .to_string()
    }

    fn task_2(&self, file: &str) -> String {
        file.lines()
            .filter(|l| {
                let nums: Vec<_> = l.split_whitespace().flat_map(str::parse::<u32>).collect();
                if is_safe(&nums) {
                    return true;
                }
                let mut new_nums = Vec::with_capacity(nums.len() - 1);
                for i in 0..nums.len() {
                    new_nums.clear();
                    new_nums.extend_from_slice(&nums[0..i]);
                    new_nums.extend_from_slice(&nums[i + 1..]);
                    if is_safe(&new_nums) {
                        return true;
                    }
                }
                false
            })
            .count()
            .to_string()
    }
}

fn is_safe(nums: &[u32]) -> bool {
    let is_increasing = nums[0] < nums[1];
    nums.windows(2).all(|w| {
        let diff = w[0].abs_diff(w[1]);
        (1..=3).contains(&diff) && (w[0] < w[1]) == is_increasing
    })
}
