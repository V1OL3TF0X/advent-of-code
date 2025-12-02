pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        let mut num_of_zeroes = 0;
        file.lines().map(parse_line).fold(50, |mut sum, value| {
            sum += value;
            if sum > 99 {
                sum -= 100;
            }
            if sum < 0 {
                sum += 100;
            }
            if sum == 0 {
                num_of_zeroes += 1;
            }
            sum
        });
        num_of_zeroes.to_string()
    }

    fn task_2(&self, file: &str) -> String {
        let mut num_of_zeroes = 0;
        file.lines()
            .map(parse_line_2)
            .fold(50, |mut sum, (value, passes_through_zero)| {
                let prev_sum = sum;
                sum += value;
                num_of_zeroes += passes_through_zero;
                if sum > 99 {
                    sum -= 100;
                    if sum != 0 && prev_sum != 0 {
                        num_of_zeroes += 1;
                    }
                }
                if sum < 0 {
                    sum += 100;

                    if sum != 0 && prev_sum != 0 {
                        num_of_zeroes += 1;
                    }
                }
                if sum == 0 {
                    num_of_zeroes += 1;
                }
                sum
            });
        num_of_zeroes.to_string()
    }
}

fn parse_line(line: &str) -> i32 {
    let (dir, num) = line.split_at(1);
    ((unsafe { num.parse::<i32>().unwrap_unchecked() }) % 100) * if dir == "R" { 1 } else { -1 }
}
fn parse_line_2(line: &str) -> (i32, i32) {
    let (dir, num) = line.split_at(1);
    let rotation = unsafe { num.parse::<i32>().unwrap_unchecked() };
    (
        (rotation % 100) * if dir == "R" { 1 } else { -1 },
        rotation / 100,
    )
}
