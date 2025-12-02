mod labirynth;
use labirynth::{Labirynth, Pipe};

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        (parse_input(file).into_iter().count() as f64 / 2.0)
            .floor()
            .to_string()
    }

    fn task_2(&self, file: &str) -> String {
        let mut lab = parse_input(file);
        lab.iter_mut()
            .for_each(|p| *p = Pipe::LabirynthPart(Box::new(p.clone())));
        lab.lines()
            .map(|l| {
                let mut lab_parts = 0;
                let mut edge_start = &Pipe::Ground;
                l.iter().fold(0, |mut inside, p| {
                    if let Pipe::LabirynthPart(inner) = p {
                        decide(inner, &mut edge_start, &mut lab_parts);
                        return inside;
                    }
                    if lab_parts % 2 != 0 {
                        inside += 1
                    }
                    inside
                })
            })
            .sum::<usize>()
            .to_string()
    }
}

fn parse_input(file: &str) -> Labirynth {
    let mut start = (usize::MAX, usize::MAX);
    let map = file
        .lines()
        .enumerate()
        .map(|(j, l)| {
            l.chars()
                .enumerate()
                .map(|(i, c)| {
                    let p = Pipe::from(c);
                    if let Pipe::Start(_) = &p {
                        start = (i, j);
                    }
                    p
                })
                .collect()
        })
        .collect();
    Labirynth::new(map, start)
}

fn decide(pipe: &Pipe, edge_start: &mut &Pipe, lab_parts: &mut usize) {
    match pipe {
        Pipe::Vertical => *lab_parts += 1,
        Pipe::Horizontal => {}
        Pipe::NEBend => {
            *edge_start = &Pipe::NEBend;
        }
        Pipe::NWBend => {
            if *edge_start == &Pipe::SEBend {
                *lab_parts += 1;
            } else if let Pipe::Start(p) = edge_start {
                if p.as_ref() == &Pipe::SEBend {
                    *lab_parts += 1;
                }
            }
        }
        Pipe::SWBend => {
            if *edge_start == &Pipe::NEBend {
                *lab_parts += 1;
            } else if let Pipe::Start(p) = edge_start {
                if p.as_ref() == &Pipe::NEBend {
                    *lab_parts += 1;
                }
            }
        }
        Pipe::SEBend => *edge_start = &Pipe::SEBend,
        Pipe::Start(p) => decide(p, edge_start, lab_parts),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::task_fns::TaskFns;

    #[test]
    fn pt_2_0() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(Solution.task_2(input), "10")
    }
    #[test]
    fn pt_2_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(Solution.task_2(input), "4")
    }
    #[test]
    fn pt_2_2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(Solution.task_2(input), "8")
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    None,
}

impl From<&Pipe> for Dir {
    fn from(value: &Pipe) -> Self {
        match value {
            &Pipe::NEBend | &Pipe::NWBend => Dir::Up,
            &Pipe::SWBend | &Pipe::SEBend => Dir::Down,
            _ => Dir::None,
        }
    }
}
