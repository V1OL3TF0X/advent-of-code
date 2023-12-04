use super::utils::{is_digit, to_str, Gear};

pub fn task_2(file: &str) -> String {
    let (bytes, astrices) = get_bytes_arr_and_astrix_pos(file);
    let last_line_ind = bytes.len() - 1;
    let last_line_char = bytes[0].len() - 1;
    let gears = astrices
        .iter()
        .fold(vec![], |mut gears, &(astrix_x, astrix_y)| {
            let mut found = vec![];
            let can_check_prev = astrix_y != 0;
            let can_check_next = astrix_y != last_line_char;
            if astrix_x != 0 {
                //check top tow
                if is_digit(&bytes[astrix_x - 1][astrix_y]) {
                    // .d. or dd. or .dd or ddd -> one number
                    found.push((astrix_x - 1, astrix_y));
                } else {
                    if can_check_prev && is_digit(&bytes[astrix_x - 1][astrix_y - 1]) {
                        found.push((astrix_x - 1, astrix_y - 1));
                    }
                    if can_check_next && is_digit(&bytes[astrix_x - 1][astrix_y + 1]) {
                        found.push((astrix_x - 1, astrix_y + 1));
                    }
                }
            }
            if can_check_prev && is_digit(&bytes[astrix_x][astrix_y - 1]) {
                if found.len() == 2 {
                    // more than two - not a gear
                    return gears;
                } else {
                    // d*?
                    found.push((astrix_x, astrix_y - 1))
                }
            }
            if can_check_next && is_digit(&bytes[astrix_x][astrix_y + 1]) {
                if found.len() == 2 {
                    // more than two - not a gear
                    return gears;
                } else {
                    // ?*d
                    found.push((astrix_x, astrix_y + 1))
                }
            }
            if astrix_x != last_line_ind {
                //check top tow
                if is_digit(&bytes[astrix_x + 1][astrix_y]) {
                    // .d. or dd. or .dd or ddd -> one number
                    found.push((astrix_x + 1, astrix_y));
                } else {
                    if can_check_prev && is_digit(&bytes[astrix_x + 1][astrix_y - 1]) {
                        found.push((astrix_x + 1, astrix_y - 1));
                    }
                    if can_check_next && is_digit(&bytes[astrix_x + 1][astrix_y + 1]) {
                        found.push((astrix_x + 1, astrix_y + 1));
                    }
                }
            }
            if found.len() == 2 {
                gears.push(Gear::from(found));
            }
            gears
        });
    let sum: u32 = gears.iter().map(|g| get_ratio(g, &bytes)).sum();
    sum.to_string()
}

fn get_bytes_arr_and_astrix_pos(file: &str) -> (Vec<&[u8]>, Vec<(usize, usize)>) {
    let mut astrix_pos = vec![];
    let bytes_arr: Vec<&[u8]> = file
        .lines()
        .enumerate()
        .map(|(i, line)| {
            astrix_pos.extend(
                line.as_bytes()
                    .iter()
                    .enumerate()
                    .filter(|&(_, c)| c == &b'*')
                    .map(|(j, _)| (i, j)),
            );
            line.as_bytes()
        })
        .collect();
    (bytes_arr, astrix_pos)
}

fn get_ratio(gear: &Gear, bytes: &[&[u8]]) -> u32 {
    let line_end = bytes[0].len() - 1;
    let mut ratio = 1;
    for (x, y) in gear.nums_adj {
        let mut start = y;
        let mut end = y;
        while start != 0 && is_digit(&bytes[x][start - 1]) {
            start -= 1;
        }
        while end != line_end && is_digit(&bytes[x][end + 1]) {
            end += 1;
        }
        ratio *= to_str(x, start, end, bytes).parse::<u32>().unwrap();
    }
    ratio
}
