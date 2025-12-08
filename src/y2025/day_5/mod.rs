use crate::task_fns::SolveMode;
use std::str::FromStr;

pub struct Solution;

impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
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

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        // SAFETY - valid input
        let db = unsafe {
            let (db, _) = file.split_once("\r\n\r\n").unwrap_unchecked();
            Database::from_str(db).unwrap_unchecked()
        };

        db.ranges
            .into_iter()
            .fold(vec![], |mut fresh: Vec<(u128, u128)>, range| {
                let mut has_intersections = false;
                let resulting_range = fresh
                    .iter()
                    .filter(|target| target.intersects(&range))
                    .inspect(|_| has_intersections = true)
                    .fold(range, |res, join| (res.0.min(join.0), res.1.max(join.1)));
                if has_intersections {
                    fresh.retain(|el| !el.intersects(&range));
                }
                fresh.push(resulting_range);
                fresh
            })
            .into_iter()
            .fold(0, |count, (from, to)| count + to - from + 1)
            .to_string()
    }
}

trait Intersects {
    fn intersects(&self, other: &Self) -> bool;
}

impl Intersects for (u128, u128) {
    fn intersects(&self, other: &Self) -> bool {
        !(self.0 > other.1 || other.0 > self.1)
    }
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
