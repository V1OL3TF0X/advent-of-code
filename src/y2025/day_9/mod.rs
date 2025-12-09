pub struct Solution;
use std::str::FromStr;

use geo::{coord, point, Contains, Coord, LineString, Polygon, Rect};
use itertools::Itertools;

use crate::{plane::Point, task_fns::SolveMode};

impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str, _: SolveMode) -> String {
        unsafe {
            file.lines()
                // SAFETY - valid input
                .map(|l| Point::from_str(l).unwrap_unchecked())
                .tuple_combinations()
                .map(|(p1, p2)| (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1))
                .max()
                // SAFETY - at least one pair of vertices
                .unwrap_unchecked()
                .to_string()
        }
    }

    fn task_2(&self, file: &str, _: SolveMode) -> String {
        let corners = file
            .lines()
            .map(|l| unsafe { Point::from_str(l).unwrap_unchecked() })
            .map(Coord::<f64>::from)
            .collect_vec();

        let valid_area = Polygon::new(LineString::from_iter(corners), vec![]);

        valid_area
            .exterior()
            .coords()
            .collect_vec()
            .into_iter()
            .tuple_combinations()
            .flat_map(|(p1, p2)| {
                if !valid_area.contains(&Rect::new(*p1, *p2)) {
                    return None;
                }
                Some(
                    ((p1.x as i64).abs_diff(p2.x as i64) + 1)
                        * ((p1.y as i64).abs_diff(p2.y as i64) + 1),
                )
            })
            .max()
            .unwrap_or(0)
            .to_string()
    }
}
