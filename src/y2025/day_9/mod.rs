pub struct Solution;
use std::str::FromStr;

use geo::{Contains, Coord, CoordNum, CoordsIter, Line, LineString, LinesIter, Polygon, Rect};
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
                let rect = Rect::new(*p1, *p2);

                if rect.coords_iter().any(|c| !valid_area.contains(&c)) {
                    return None;
                }
                if valid_area.exterior().coords().any(|c| rect.contains(c)) {
                    return None;
                }
                if valid_area
                    .exterior()
                    .lines()
                    .cartesian_product(rect.lines_iter())
                    .any(are_lines_intersecting)
                {
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

fn are_lines_intersecting<T: CoordNum>(lines: (Line<T>, Line<T>)) -> bool {
    let (p_1_l_1, p_2_l_1) = lines.0.points();
    let (p_1_l_2, p_2_l_2) = lines.0.points();
    let (vert, horiz) = if p_1_l_1.x() == p_2_l_1.x() {
        ((p_1_l_1, p_2_l_1), (p_1_l_2, p_2_l_2))
    } else {
        ((p_1_l_2, p_2_l_2), (p_1_l_1, p_2_l_1))
    };
    let vert_min_x = if vert.0.x() < vert.1.x() {
        vert.0.x()
    } else {
        vert.1.x()
    };
    let (horiz_min_x, horiz_max_x) = if horiz.0.x() < horiz.1.x() {
        (horiz.0.x(), horiz.1.x())
    } else {
        (horiz.1.x(), horiz.0.x())
    };

    if vert_min_x <= horiz_min_x || vert_min_x >= horiz_max_x {
        return false;
    }
    let (vert_min_y, vert_max_y) = if vert.0.y() < vert.1.y() {
        (vert.0.y(), vert.1.y())
    } else {
        (vert.1.y(), vert.0.y())
    };
    let (horiz_min_y, horiz_may_y) = if horiz.0.y() < horiz.1.y() {
        (horiz.0.y(), horiz.1.y())
    } else {
        (horiz.1.y(), horiz.0.y())
    };
    vert_min_y < horiz_min_y && vert_max_y > horiz_may_y
}
