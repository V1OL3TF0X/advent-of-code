use std::collections::HashSet;

struct State<NextNodesCollector: IntoIterator<Item = (usize, usize)>>(Vec<NextNodesCollector>);

impl<NextStatesCollector: IntoIterator<Item = (usize, usize)>> FromIterator<NextStatesCollector>
    for State<NextStatesCollector>
{
    fn from_iter<T: IntoIterator<Item = NextStatesCollector>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

fn solve<
    T: IntoIterator<Item = (usize, usize)>
        + FromIterator<(usize, usize)>
        + Default
        + Extend<(usize, usize)>,
>(
    file: &str,
) -> String
where
    T::IntoIter: ExactSizeIterator,
{
    let mut state: State<T> = State(vec![]);
    let map = file
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| (x, c.to_digit(10).unwrap()))
                .inspect(|(x, d)| {
                    if *d == 0 {
                        state.0.push(T::from_iter([(*x, y)]));
                    }
                })
                .map(|(_, d)| d)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for next in 1..=9 {
        state.0.iter_mut().for_each(|pos| {
            let mut new = T::default();
            std::mem::swap(pos, &mut new);
            new.into_iter().for_each(|(x, y)| {
                let mut next_nodes = vec![];
                if y < map.len() - 1 && map[y + 1][x] == next {
                    next_nodes.push((x, y + 1));
                }
                if y > 0 && map[y - 1][x] == next {
                    next_nodes.push((x, y - 1));
                }
                if x < map[0].len() - 1 && map[y][x + 1] == next {
                    next_nodes.push((x + 1, y));
                }
                if x > 0 && map[y][x - 1] == next {
                    next_nodes.push((x - 1, y));
                }
                pos.extend(next_nodes.into_iter())
            });
        });
    }
    state
        .0
        .into_iter()
        .map(|b| b.into_iter().len())
        .sum::<usize>()
        .to_string()
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        solve::<HashSet<_>>(file)
    }

    fn task_2(&self, file: &str) -> String {
        solve::<Vec<_>>(file)
    }
}

