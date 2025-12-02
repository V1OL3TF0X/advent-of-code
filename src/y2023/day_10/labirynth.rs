use std::{fmt::Debug, marker::PhantomData, ops::Index, slice::Iter};
pub struct Labirynth {
    map: Vec<Vec<Pipe>>,
    start: (usize, usize),
}

impl Debug for Labirynth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.map {
            for p in l {
                write!(f, "{p:?}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Labirynth {
    pub fn new(mut map: Vec<Vec<Pipe>>, start: (usize, usize)) -> Self {
        let mut directions = vec![];
        let start = (start.0 as i16, start.1 as i16);
        for (i, j) in DIRECTIONS {
            if (start.0 == 0 && i == -1) || (start.1 == 0 && j == -1) {
                continue;
            }
            if map[(start.1 + j) as usize][(start.0 + i) as usize].connects_in_dir((i, j)) {
                // find first pipe connected to start
                directions.push((i, j));
            }
        }
        let start = (start.0 as usize, start.1 as usize);
        map[start.1][start.0] = Pipe::Start(Box::new(Pipe::from(directions)));
        Self { map, start }
    }
    pub fn into_iter(self) -> LabirynthIter {
        let mut direction = None;
        let start = (self.start.0 as i16, self.start.1 as i16);
        for (i, j) in DIRECTIONS {
            if (self.start.0 == 0 && i == -1) || (self.start.1 == 0 && j == -1) {
                continue;
            }
            if self.map[(start.1 + j) as usize][(start.0 + i) as usize].connects_in_dir((i, j)) {
                // find first pipe connected to start
                direction = Some((i, j));
                break;
            }
        }
        LabirynthIter {
            map: self.map,
            current: start,
            direction,
        }
    }
    pub fn iter_mut(&mut self) -> LabirynthMutIter<'_> {
        let mut direction = None;
        let start = (self.start.0 as i16, self.start.1 as i16);
        for (i, j) in DIRECTIONS {
            if (self.start.0 == 0 && i == -1) || (self.start.1 == 0 && j == -1) {
                continue;
            }
            if self.map[(start.1 + j) as usize][(start.0 + i) as usize].connects_in_dir((i, j)) {
                // find first pipe connected to start
                direction = Some((i, j));
                break;
            }
        }
        LabirynthMutIter {
            labirynth: self,
            current: start,
            direction,
            _marker: PhantomData,
        }
    }
    pub fn lines(&self) -> Iter<'_, Vec<Pipe>> {
        self.map.iter()
    }
}

pub struct LabirynthMutIter<'l> {
    labirynth: *mut Labirynth,
    current: (i16, i16),
    direction: Option<(i16, i16)>,
    _marker: PhantomData<&'l mut Pipe>,
}

impl<'l> Iterator for LabirynthMutIter<'l> {
    type Item = &'l mut Pipe;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.direction?;
        self.current = (self.current.0 + x, self.current.1 + y);
        unsafe {
            let current_pipe: &Pipe = (&(*self.labirynth).map)
                .index(self.current.1 as usize)
                .index(self.current.0 as usize);
            self.direction = if let Pipe::Start(_) = current_pipe {
                None
            } else {
                Some(current_pipe.get_next_dir((x, y)))
            };
            // safety -> we get each item only once
            Some(
                (&mut (*self.labirynth).map)
                    .get_unchecked_mut(self.current.1 as usize)
                    .get_unchecked_mut(self.current.0 as usize),
            )
        }
    }
}
// pt 2 -> scanlinie, swap loop with I (or smth)

pub struct LabirynthIter {
    map: Vec<Vec<Pipe>>,
    current: (i16, i16),
    direction: Option<(i16, i16)>,
}

impl Iterator for LabirynthIter {
    type Item = Pipe;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.direction?;
        self.current = (self.current.0 + x, self.current.1 + y);
        let current_pipe = self.map[self.current.1 as usize][self.current.0 as usize].clone();
        self.direction = if let Pipe::Start(_) = current_pipe {
            None
        } else {
            Some(current_pipe.get_next_dir((x, y)))
        };
        Some(current_pipe)
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
    Ground,
    Start(Box<Pipe>),
    LabirynthPart(Box<Pipe>),
}

impl Pipe {
    /// function telling us, if we connected to self when encountering it after moving with given direction
    ///
    /// # Examples
    /// connects_in_dir((0, -1)) means we encountered self while moving in (0, -1) direction from a pipe
    pub fn connects_in_dir(&self, (x, y): (i16, i16)) -> bool {
        match self {
            Pipe::Vertical => y != 0,
            Pipe::Horizontal => x != 0,
            //              move down  move left
            Pipe::NEBend => y == -1 || x == -1,
            //              move down  move right
            Pipe::NWBend => y == -1 || x == 1,
            //              move up  move right
            Pipe::SWBend => y == 1 || x == 1,
            //              move up  move left
            Pipe::SEBend => y == 1 || x == -1,
            Pipe::Ground => false,
            Pipe::Start(p) => p.connects_in_dir((x, y)),
            Pipe::LabirynthPart(p) => p.connects_in_dir((x, y)),
        }
    }
    pub fn get_next_dir(&self, from_dir: (i16, i16)) -> (i16, i16) {
        let (x, y) = from_dir;
        match self {
            Pipe::Vertical => (0, y),
            Pipe::Horizontal => (x, 0),
            Pipe::NEBend => {
                if y == 1 {
                    (1, 0)
                } else {
                    (0, -1)
                }
            }
            Pipe::NWBend => {
                if y == 1 {
                    (-1, 0)
                } else {
                    (0, -1)
                }
            }
            Pipe::SWBend => {
                if y == -1 {
                    (-1, 0)
                } else {
                    (0, 1)
                }
            }
            Pipe::SEBend => {
                if y == -1 {
                    (1, 0)
                } else {
                    (0, 1)
                }
            }
            Pipe::Ground => unreachable!("Ground cannot be on path!"),
            Pipe::Start(_) => unreachable!("Start is only at the strat of the path!"),
            Pipe::LabirynthPart(_) => unreachable!(),
        }
    }
    fn symbol(&self) -> char {
        match self {
            Self::Vertical => '|',
            Self::Horizontal => '─',
            Self::NEBend => '└',
            Self::NWBend => '┘',
            Self::SWBend => '┐',
            Self::SEBend => '┌',
            Pipe::Ground => ' ',
            Pipe::Start(p) => p.symbol(),
            Pipe::LabirynthPart(p) => p.symbol(),
        }
    }
}

impl Debug for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = self.symbol();
        if let Pipe::LabirynthPart(p) = &self {
            if let Pipe::Start(_) = p.as_ref() {
                write!(f, "\x1B[92m{d}\x1B[39m")
            } else {
                write!(f, "\x1B[31m{d}\x1B[39m")
            }
        } else if let Pipe::Start(_) = &self {
            write!(f, "\x1B[92m{d}\x1B[39m")
        } else {
            write!(f, "{d}")
        }
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NEBend,
            'J' => Self::NWBend,
            '7' => Self::SWBend,
            'F' => Self::SEBend,
            '.' => Self::Ground,
            'S' => Self::Start(Box::new(Pipe::Ground)),
            c => unreachable!("characteer {c} is not a valid pipe character!"),
        }
    }
}

impl From<Vec<(i16, i16)>> for Pipe {
    fn from(value: Vec<(i16, i16)>) -> Self {
        // we check only one set, because DIRECTIONS has them specified in a particular order
        match (value[0], value[1]) {
            // move down and move up
            ((0, -1), (0, 1)) => Self::Vertical,
            // move left and move right
            ((-1, 0), (1, 0)) => Self::Horizontal,
            // move left and move down
            ((-1, 0), (0, 1)) => Self::NEBend,
            // move right and move down
            ((1, 0), (0, 1)) => Self::NWBend,
            // move right and move up
            ((1, 0), (0, -1)) => Self::SWBend,
            // move left and move up
            ((-1, 0), (0, -1)) => Self::SEBend,
            _ => unreachable!("value {value:?} is not a valid pipe direction!"),
        }
    }
}

const DIRECTIONS: [(i16, i16); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
