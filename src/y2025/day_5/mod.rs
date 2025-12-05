use std::{cmp::Ordering, str::FromStr};

pub struct Solution;

impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        // SAFETY - valid input
        let (db, to_check) = unsafe {
            let (db, to_check) = file.split_once("\r\n\r\n").unwrap_unchecked();
            (Database::from_str(db).unwrap_unchecked(), to_check)
        };
        to_check
            .lines()
            .flat_map(str::parse::<u128>)
            .filter(|ingredient| {
                let includes = db
                    .ranges
                    .iter()
                    .find(|(from, to)| from <= ingredient && ingredient <= to);
                println!("{includes:?} includes {ingredient}");
                includes.is_some()
            })
            .count()
            .to_string()
    }

    fn task_2(&self, file: &str) -> String {
        let db = unsafe {
            let (db, _) = file.split_once("\r\n\r\n").unwrap_unchecked();
            Database::from_str(db).unwrap_unchecked()
        };

        db.ranges
            .into_iter()
            .fold(vec![], |mut fresh, range| {
                let (indices_to_remove, resulting_range) = fresh
                    .iter()
                    .enumerate()
                    .filter(|(_, target)| intersects(target, &range))
                    .fold((vec![], range), |(mut index_vec, res), (i, join)| {
                        index_vec.push(i);
                        (index_vec, (res.0.min(join.0), res.1.max(join.1)))
                    });
                if !indices_to_remove.is_empty() {
                    fresh = remove_sorted_indices(fresh, indices_to_remove);
                }
                fresh.push(resulting_range);
                fresh
            })
            .into_iter()
            .fold(0, |count, (from, to)| count + to - from + 1)
            .to_string()
    }
}

fn intersects(a: &(u128, u128), b: &(u128, u128)) -> bool {
    !(b.0 > a.1 || a.0 > b.1)
}

#[derive(Debug)]
struct Database {
    ranges: Vec<(u128, u128)>,
}

impl FromStr for Database {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            ranges: s
                .lines()
                .flat_map(|l| l.split_once('-').ok_or("no delimiter found in range"))
                .map(|(from, to)| {
                    from.parse()
                        .and_then(|parsed_from| Ok((parsed_from, to.parse()?)))
                        .map_err(|_| "not a number in range start / end")
                })
                .collect::<Result<Vec<_>, &'static str>>()?,
        })
    }
}

// credit: https://users.rust-lang.org/t/removing-multiple-indices-from-a-vector/65599/4
fn remove_sorted_indices<T>(
    v: impl IntoIterator<Item = T>,
    indices: impl IntoIterator<Item = usize>,
) -> Vec<T> {
    let v = v.into_iter();
    let mut indices = indices.into_iter();
    let mut i = match indices.next() {
        None => return v.collect(),
        Some(i) => i,
    };
    let (min, max) = v.size_hint();
    let mut result = Vec::with_capacity(max.unwrap_or(min));

    for (j, x) in v.into_iter().enumerate() {
        if j == i {
            if let Some(idx) = indices.next() {
                i = idx;
            }
        } else {
            result.push(x);
        }
    }

    result
}
