use std::collections::HashMap;

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        let mut lines = file.lines();

        let mut rules_by_prec_page = HashMap::new();
        let mut rules_by_adv_page = HashMap::new();
        (&mut lines)
            .take_while(|l| !l.is_empty())
            .map(|l| {
                l.split_once('|')
                    .map(|(x, y)| (x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap()))
                    .unwrap()
            })
            .for_each(|(x, y)| {
                rules_by_prec_page
                    .entry(x)
                    .and_modify(|a: &mut Vec<_>| a.push(y))
                    .or_insert(vec![y]);
                rules_by_adv_page
                    .entry(y)
                    .and_modify(|a: &mut Vec<_>| a.push(x))
                    .or_insert(vec![x]);
            });
        lines
            .fold(0, |sum, line| {
                let digits = line
                    .split(',')
                    .flat_map(|d| d.parse::<u16>())
                    .collect::<Vec<_>>();
                if !adheres_to_rules(&digits, &rules_by_adv_page, &rules_by_prec_page) {
                    return sum;
                }
                sum + digits[digits.len() / 2]
            })
            .to_string()
    }

    fn task_2(&self, file: &str) -> String {
        let mut lines = file.lines();

        let mut rules_by_prec_page = HashMap::new();
        let mut rules_by_adv_page = HashMap::new();
        (&mut lines)
            .take_while(|l| !l.is_empty())
            .map(|l| {
                l.split_once('|')
                    .map(|(x, y)| (x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap()))
                    .unwrap()
            })
            .for_each(|(x, y)| {
                rules_by_prec_page
                    .entry(x)
                    .and_modify(|a: &mut Vec<_>| a.push(y))
                    .or_insert(vec![y]);
                rules_by_adv_page
                    .entry(y)
                    .and_modify(|a: &mut Vec<_>| a.push(x))
                    .or_insert(vec![x]);
            });
        lines
            .fold(0, |sum, line| {
                let digits = line
                    .split(',')
                    .flat_map(|d| d.parse::<u16>())
                    .collect::<Vec<_>>();
                if adheres_to_rules(&digits, &rules_by_adv_page, &rules_by_prec_page) {
                    return sum;
                }
                let mut new_digits = vec![];
                digits.into_iter().for_each(|d| {
                    new_digits.push(d);
                    let mut i = new_digits.len() - 1;
                    while !is_in_right_place(
                        &new_digits,
                        &rules_by_adv_page,
                        &rules_by_prec_page,
                        i,
                    ) {
                        new_digits.swap(i, i - 1);
                        i -= 1;
                    }
                });
                sum + new_digits[new_digits.len() / 2]
            })
            .to_string()
    }
}

fn adheres_to_rules(
    line: &[u16],
    prev_map: &HashMap<u16, Vec<u16>>,
    next_map: &HashMap<u16, Vec<u16>>,
) -> bool {
    line.iter()
        .enumerate()
        .all(|(i, _)| is_in_right_place(line, prev_map, next_map, i))
}
fn is_in_right_place(
    line: &[u16],
    prev_map: &HashMap<u16, Vec<u16>>,
    next_map: &HashMap<u16, Vec<u16>>,
    i: usize,
) -> bool {
    let d = &line[i];
    prev_map.get(d).map_or(true, |prev_list| {
        prev_list
            .iter()
            .all(|prev| line.iter().position(|e| e == prev).map_or(true, |j| j < i))
    }) && next_map.get(d).map_or(true, |next_list| {
        next_list
            .iter()
            .all(|next| line.iter().position(|e| e == next).map_or(true, |j| j > i))
    })
}
