use super::utils::{adj_symbol, is_digit, to_str, Num};

pub fn task_1(file: &str) -> String {
    let (bytes_arr, nums) = get_bytes_arr_and_num_ind(file);
    let last_line_ind = bytes_arr.len() - 1;
    let last_line_char = bytes_arr[0].len() - 1;
    nums.into_iter()
        .filter(|num| {
            let (start_x, end_x) = match num.line {
                0 => (0, 1),
                n if n == last_line_ind => (last_line_ind - 1, num.line),
                _ => (num.line - 1, num.line + 1),
            };
            let (start_y, end_y) = match num.start_ind {
                0 => (0, 0),
                n => (n - 1, num.start_ind),
            };
            if adj_symbol(&bytes_arr, start_x, end_x, start_y, end_y) {
                // XX
                // Xs <- start of digit
                // XX
                return true;
            }
            for d in (num.start_ind + 1)..num.end_ind {
                // ...X...
                // DDDdDD
                // ...X...
                if adj_symbol(&bytes_arr, start_x, end_x, d, d) {
                    return true;
                }
            }
            let end_y = if num.end_ind == last_line_char {
                last_line_char
            } else {
                num.end_ind + 1
            };
            // XX
            // eX
            // XX
            adj_symbol(&bytes_arr, start_x, end_x, num.end_ind, end_y)
        })
        .map(|num| {
            to_str(num.line, num.start_ind, num.end_ind, &bytes_arr)
                .parse::<u32>()
                .unwrap()
        })
        .sum::<u32>()
        .to_string()
}

fn get_bytes_arr_and_num_ind(file: &str) -> (Vec<&[u8]>, Vec<Num>) {
    let mut nums = vec![];
    let mut current_start = usize::MAX;
    let mut current_len = 0;
    let bytes_arr: Vec<&[u8]> = file
        .lines()
        .enumerate()
        .map(|(i, line)| {
            // .....456
            // ....... <- we're on this line
            if current_start != usize::MAX {
                nums.push(Num {
                    end_ind: line.len() - 1,
                    line: i - 1,
                    start_ind: current_start,
                });
                current_start = usize::MAX;
            }
            line.as_bytes().iter().enumerate().for_each(|(j, c)| {
                if is_digit(c) {
                    if current_start == usize::MAX {
                        current_start = j;
                    } else {
                        current_len += 1;
                    }
                } else if current_start != usize::MAX {
                    nums.push(Num {
                        end_ind: j - 1,
                        line: i,
                        start_ind: current_start,
                    });
                    current_start = usize::MAX;
                }
            });
            line.as_bytes()
        })
        .collect();
    (bytes_arr, nums)
}
