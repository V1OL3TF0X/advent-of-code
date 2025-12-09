use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
    str::FromStr,
};

use geo::Coord;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

impl Direction {
    pub const fn rev(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
        }
    }
    pub fn turns(&self, other: &Self) -> usize {
        if self == other {
            return 0;
        }
        if *self == other.rev() {
            return 2;
        }
        1
    }
    pub fn one_turn_away(&self) -> [Self; 2] {
        match self {
            Self::Left | Self::Right => [Self::Top, Self::Bottom],
            Self::Top | Self::Bottom => [Self::Left, Self::Right],
        }
    }
}
impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
            Direction::Top => write!(f, "^"),
            Direction::Bottom => write!(f, "v"),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Top,
            '>' => Self::Right,
            'v' => Self::Bottom,
            '<' => Self::Left,
            v => panic!("{v} is not a valid direction!"),
        }
    }
}

impl From<Point> for Direction {
    fn from(value: Point) -> Self {
        match (value.0, value.1) {
            (0, -1) => Self::Top,
            (1, 0) => Self::Right,
            (0, 1) => Self::Bottom,
            (-1, 0) => Self::Left,
            v => panic!("{v:?} is not a valid direction!"),
        }
    }
}

impl From<&Direction> for Point {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Top => Self(0, -1),
            Direction::Right => Self(1, 0),
            Direction::Bottom => Self(0, 1),
            Direction::Left => Self(-1, 0),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point(pub i64, pub i64);

impl Point {
    pub const fn new(x: usize, y: usize) -> Self {
        Self(x as i64, y as i64)
    }
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(',')
            .ok_or("no , in string to create point")
            .and_then(|(a, b)| {
                Ok((
                    a.parse().map_err(|_| "couldn't parse first coordinate")?,
                    b.parse().map_err(|_| "couldn't parse second coordinate")?,
                ))
            })
            .map(Point::from)
    }
}

impl From<Point> for Coord<i64> {
    fn from(value: Point) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<Point> for Coord<f64> {
    fn from(value: Point) -> Self {
        Self {
            x: value.0 as f64,
            y: value.1 as f64,
        }
    }
}

impl From<(i64, i64)> for Point {
    fn from(value: (i64, i64)) -> Self {
        Self(value.0, value.1)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Add<&Direction> for &Point {
    type Output = Point;

    fn add(self, rhs: &Direction) -> Self::Output {
        self + &Point::from(rhs)
    }
}
impl Add<Direction> for &Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        self + &rhs
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<&Self> for Point {
    fn add_assign(&mut self, rhs: &Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        *self += &Point::from(&rhs)
    }
}
