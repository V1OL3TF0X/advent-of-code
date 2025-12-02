use core::panic;
use std::{char, fmt::Debug};

use rustc_hash::FxHashSet;

use crate::plane::{Direction, Point};

#[derive(PartialEq, Eq, Default)]
enum TileTask1 {
    Box,
    Wall,
    #[default]
    Floor,
}

impl<'a> From<&'a TileTask1> for char {
    fn from(value: &TileTask1) -> Self {
        match value {
            TileTask1::Box => 'O',
            TileTask1::Wall => '#',
            TileTask1::Floor => '.',
        }
    }
}

trait Tileable: Sized + Default {
    fn is_box(&self) -> bool;
    fn move_robot(wh: &mut Warehouse<Self>, d: Direction, blocked: &mut Vec<Direction>);
    fn parse(v: char) -> Vec<Self>;
    fn robot_pos(x: usize, y: usize) -> Point {
        Point(x as i64, y as i64)
    }
}

impl Tileable for TileTask1 {
    fn is_box(&self) -> bool {
        *self == Self::Box
    }

    fn parse(v: char) -> Vec<Self> {
        vec![match v {
            '#' => Self::Wall,
            '.' | '@' => Self::Floor,
            'O' => Self::Box,
            _ => panic!("not a valid tile"),
        }]
    }

    fn move_robot(warehouse: &mut Warehouse<Self>, d: Direction, blocked: &mut Vec<Direction>) {
        let movement: Point = (&d).into();
        let considered_pos = &warehouse.robot + &movement;
        match warehouse.plan[considered_pos.1 as usize][considered_pos.0 as usize] {
            TileTask1::Box => {
                let mut find = &considered_pos + &movement;
                while *warehouse.get(&find) == TileTask1::Box {
                    find += &movement;
                }
                if *warehouse.get(&find) == TileTask1::Wall {
                    blocked.push(d);
                } else {
                    warehouse.set(&find, TileTask1::Box);
                    warehouse.robot += &movement;
                    warehouse.set_robot(TileTask1::Floor);
                    blocked.clear();
                }
            }
            TileTask1::Wall => blocked.push(d),
            TileTask1::Floor => {
                warehouse.robot = considered_pos;
                blocked.clear();
            }
        }
    }
}

struct Warehouse<T> {
    robot: Point,
    plan: Vec<Vec<T>>,
}
impl<T> Warehouse<T>
where
    T: Tileable,
    for<'a> char: From<&'a T>,
{
    fn from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut robot = Point(-1, -1);
        let plan = lines
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '@' {
                            robot = T::robot_pos(x, y);
                        }
                        c
                    })
                    .flat_map(T::parse)
                    .collect()
            })
            .collect();
        Self { plan, robot }
    }
    fn get(&self, p: &Point) -> &T {
        &self.plan[p.1 as usize][p.0 as usize]
    }
    fn get_mut(&mut self, p: &Point) -> &mut T {
        &mut self.plan[p.1 as usize][p.0 as usize]
    }
    fn set(&mut self, p: &Point, v: T) {
        self.plan[p.1 as usize][p.0 as usize] = v;
    }
    fn set_robot(&mut self, v: T) {
        self.plan[self.robot.1 as usize][self.robot.0 as usize] = v;
    }
    fn swap(&mut self, p1: &Point, p2: &Point) {
        let mut placeholder = T::default();
        std::mem::swap(&mut placeholder, self.get_mut(p1));
        std::mem::swap(&mut placeholder, self.get_mut(p2));
        std::mem::swap(&mut placeholder, self.get_mut(p1));
    }
    fn sum_of_gps(&self) -> u64 {
        self.plan
            .iter()
            .enumerate()
            .map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .filter(|(_, t)| t.is_box())
                    .map(|(x, _)| (y * 100 + x) as u64)
                    .sum::<u64>()
            })
            .sum()
    }
}

impl<T> Debug for Warehouse<T>
where
    char: for<'a> From<&'a T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.plan.iter().enumerate() {
            let y = y as i64;
            for (x, t) in line.iter().enumerate() {
                if x as i64 == self.robot.0 && y == self.robot.1 {
                    write!(f, "@")?;
                } else {
                    write!(f, "{}", char::from(t))?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve<T: Tileable>(file: &str) -> String
where
    char: for<'a> From<&'a T>,
{
    let mut lines = file.lines();
    let mut warehouse = Warehouse::<T>::from_lines((&mut lines).take_while(|l| !l.is_empty()));
    let mut blocked = Vec::with_capacity(4);

    lines
        .flat_map(|l| l.chars().map(Direction::from))
        .for_each(|d| {
            if blocked.iter().any(|b| *b == d) {
                return;
            }
            T::move_robot(&mut warehouse, d, &mut blocked);
        });
    println!("{warehouse:?}");
    warehouse.sum_of_gps().to_string()
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        solve::<TileTask1>(file)
    }

    fn task_2(&self, file: &str) -> String {
        solve::<TileTask2>(file)
    }
}

#[derive(PartialEq, Eq, Clone, Default)]
enum TileTask2 {
    BoxLeft,
    BoxRight,
    Wall,
    #[default]
    Floor,
}

impl<'a> From<&'a TileTask2> for char {
    fn from(value: &TileTask2) -> Self {
        match value {
            TileTask2::BoxLeft => '[',
            TileTask2::BoxRight => ']',
            TileTask2::Wall => '#',
            TileTask2::Floor => '.',
        }
    }
}

impl Tileable for TileTask2 {
    fn is_box(&self) -> bool {
        *self == Self::BoxLeft
    }

    fn parse(v: char) -> Vec<Self> {
        match v {
            '#' => vec![Self::Wall, Self::Wall],
            '.' | '@' => vec![Self::Floor, Self::Floor],
            'O' => vec![Self::BoxLeft, Self::BoxRight],
            _ => panic!("not a valid tile"),
        }
    }
    fn robot_pos(x: usize, y: usize) -> Point {
        Point(x as i64 * 2, y as i64)
    }

    fn move_robot(warehouse: &mut Warehouse<Self>, d: Direction, blocked: &mut Vec<Direction>) {
        let movement: Point = (&d).into();
        let considered_pos = &warehouse.robot + &movement;
        match warehouse.plan[considered_pos.1 as usize][considered_pos.0 as usize] {
            TileTask2::BoxLeft => {
                if can_move_box(warehouse, &considered_pos, &d) {
                    move_box(warehouse, &considered_pos, &d);
                    blocked.clear();
                } else {
                    blocked.push(d);
                }
            }
            TileTask2::BoxRight => {
                let left_corner = &considered_pos + Direction::Left;
                if can_move_box(warehouse, &left_corner, &d) {
                    move_box(warehouse, &left_corner, &d);
                    blocked.clear();
                } else {
                    blocked.push(d);
                }
            }
            TileTask2::Wall => blocked.push(d),
            TileTask2::Floor => {
                warehouse.robot = considered_pos;
                blocked.clear();
            }
        }
    }
}

fn get_next_boxes_to_check(
    warehouse: &Warehouse<TileTask2>,
    boxes: &FxHashSet<Point>,
    d: &Direction,
) -> Option<FxHashSet<Point>> {
    let mut boxes_to_check = FxHashSet::default();
    for left_corner in boxes.iter() {
        let movement = d.into();
        let mut next_left_pos = &movement + left_corner;
        match warehouse.get(&next_left_pos) {
            TileTask2::BoxLeft => {
                boxes_to_check.insert(next_left_pos);
            }
            TileTask2::BoxRight => {
                boxes_to_check.insert(&next_left_pos + Direction::Left);
            }
            TileTask2::Wall => return None,
            TileTask2::Floor => {}
        };
        next_left_pos += Direction::Right;
        match warehouse.get(&next_left_pos) {
            TileTask2::BoxLeft => {
                boxes_to_check.insert(next_left_pos);
            }
            TileTask2::BoxRight => {
                boxes_to_check.insert(&next_left_pos + Direction::Left);
            }
            TileTask2::Wall => return None,
            TileTask2::Floor => {}
        };
    }
    Some(boxes_to_check)
}

fn can_move_box(warehouse: &Warehouse<TileTask2>, left_corner: &Point, d: &Direction) -> bool {
    match d {
        Direction::Left | Direction::Right => {
            let movement = d.into();
            let mut find = left_corner + &movement;
            while matches!(
                *warehouse.get(&find),
                TileTask2::BoxLeft | TileTask2::BoxRight
            ) {
                find += &movement;
            }
            *warehouse.get(&find) != TileTask2::Wall
        }
        Direction::Top | Direction::Bottom => {
            let mut boxes = FxHashSet::default();
            boxes.insert(*left_corner);
            while let Some(next) = get_next_boxes_to_check(warehouse, &boxes, d) {
                if next.is_empty() {
                    return true;
                }
                boxes = next;
            }
            false
        }
    }
}

fn move_box(warehouse: &mut Warehouse<TileTask2>, left_corner: &Point, d: &Direction) {
    match d {
        Direction::Left | Direction::Right => {
            let first = if *d == Direction::Left {
                left_corner + Direction::Right
            } else {
                *left_corner
            };
            let mut prev = warehouse.get(&first).clone();
            let movement = d.into();
            let mut find = &first + &movement;
            while *warehouse.get(&find) != TileTask2::Floor {
                std::mem::swap(&mut prev, warehouse.get_mut(&find));
                find += &movement;
            }
            warehouse.set(&find, prev);
            warehouse.set(&first, TileTask2::Floor);
        }
        Direction::Top | Direction::Bottom => {
            let mut box_layers = vec![];
            let mut boxes = FxHashSet::default();
            boxes.insert(*left_corner);
            box_layers.push(boxes);
            while let Some(next) = get_next_boxes_to_check(warehouse, box_layers.last().unwrap(), d)
            {
                if next.is_empty() {
                    break;
                }
                box_layers.push(next);
            }
            box_layers.into_iter().rev().for_each(|layer| {
                layer.into_iter().for_each(|box_l| {
                    warehouse.swap(&box_l, &(&box_l + d));
                    warehouse.swap(
                        &(&box_l + Direction::Right),
                        &(&(&box_l + d) + Direction::Right),
                    );
                });
            });
        }
    }
    warehouse.robot += &d.into();
    warehouse.set_robot(TileTask2::Floor);
}
