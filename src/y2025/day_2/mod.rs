pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        file.split(',')
            .map(parse_range)
            .map(|(from, to)| {
                let mut start = from;
                let mut invalid_count = 0;
                while start <= to {
                    let digit_count = start.ilog10() + 1;

                    if !digit_count.is_multiple_of(2) {
                        let count = digit_count;
                        start = 10u128.pow(count) + 10u128.pow(count / 2);
                        continue;
                    }
                    let half = 10u128.pow(digit_count / 2);
                    let front = start / half;
                    let back = start % half;
                    if front == back {
                        invalid_count += start;
                        start += half + 1;
                        continue;
                    }
                    if front > back {
                        start += front - back;
                        continue;
                    }
                    start += 1;
                }
                invalid_count
            })
            .sum::<u128>()
            .to_string()
    }

    fn task_2(&self, file: &str) -> String {
        file.split(',')
            .map(parse_range)
            .map(|(from, to)| {
                let mut start = from.max(11);
                let mut invalid_count = 0;
                while start <= to {
                    if are_all_digits_same(start) {
                        invalid_count += start;
                        start += 1;
                        continue;
                    }
                    let digit_count = start.ilog10() + 1;
                    if !heuristic_can_have_repetitions(digit_count) {
                        let mut count = digit_count;
                        while !heuristic_can_have_repetitions(count) {
                            count += 1;
                        }
                        let part_number =
                            unsafe { get_prime_divisors(count).next().unwrap_unchecked() };
                        let smallest = count as f64 / part_number as f64;
                        let prev = start;
                        let leading = start / 10u128.pow(digit_count - 1);
                        let mut all_ones = all_ones_with_length(digit_count);
                        let mut skipped = leading * all_ones;
                        if skipped < prev {
                            if leading == 9 {
                                all_ones = all_ones * 10 + 1;
                                skipped = all_ones;
                            } else {
                                skipped += all_ones;
                            }
                        }
                        start = (1..=part_number).fold(0, |sum, mult| {
                            sum + 10u128.pow((smallest * mult as f64) as u32 - 1)
                        });
                        while skipped < to && skipped < start {
                            invalid_count += skipped;
                            if skipped % 10 == 9 {
                                all_ones = all_ones * 10 + 1;
                                skipped = all_ones;
                            } else {
                                skipped += all_ones;
                            }
                        }
                        continue;
                    }
                    if get_prime_divisors(digit_count).any(|div| {
                        let part = 10u128.pow(digit_count / div);
                        let mut num = start / part;
                        let to_find = start % part;
                        while num > 0 {
                            let to_comp = num % part;
                            if to_comp != to_find {
                                return false;
                            }
                            num /= part;
                        }
                        true
                    }) {
                        invalid_count += start;
                    }
                    start += 1;
                }
                invalid_count
            })
            .sum::<u128>()
            .to_string()
    }
}

fn parse_range(range: &str) -> (u128, u128) {
    // SAFETY  - valid input
    let parsed = unsafe {
        range
            .split_once('-')
            .map(|(f, t)| {
                (
                    f.trim_start().parse().unwrap_unchecked(),
                    t.trim_end().parse().unwrap_unchecked(),
                )
            })
            .unwrap_unchecked()
    };
    parsed
}
const FIRST_PRIMES: [u32; 7] = [2, 3, 5, 7, 11, 13, 17];
fn heuristic_can_have_repetitions(num: u32) -> bool {
    get_prime_divisors(num).next().is_some()
}

fn get_prime_divisors(num: u32) -> impl Iterator<Item = u32> {
    FIRST_PRIMES
        .iter()
        .filter(move |prime| num.is_multiple_of(**prime) && num > **prime)
        .copied()
}

fn are_all_digits_same(mut num: u128) -> bool {
    let last = num % 10;
    while num != 0 {
        if num % 10 != last {
            return false;
        }
        num /= 10;
    }
    true
}
fn all_ones_with_length(n: u32) -> u128 {
    let mut res = 1;
    for _ in 1..n {
        res *= 10;
        res += 1;
    }
    res
}
