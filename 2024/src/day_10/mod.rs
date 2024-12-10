use std::collections::HashSet;

struct State<T: IntoIterator<Item = (usize, usize)>>(Vec<T>);

fn solve<T: IntoIterator<Item = (usize, usize)> + FromIterator<(usize, usize)>>(
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
        state = State(
            state
                .0
                .into_iter()
                .map(|pos| {
                    pos.into_iter()
                        .flat_map(|(x, y)| {
                            let mut next_nnodes = vec![];
                            if y < map.len() - 1 && map[y + 1][x] == next {
                                next_nnodes.push((x, y + 1));
                            }
                            if y > 0 && map[y - 1][x] == next {
                                next_nnodes.push((x, y - 1));
                            }
                            if x < map[0].len() - 1 && map[y][x + 1] == next {
                                next_nnodes.push((x + 1, y));
                            }
                            if x > 0 && map[y][x - 1] == next {
                                next_nnodes.push((x - 1, y));
                            }
                            next_nnodes
                        })
                        .collect()
                })
                .collect(),
        );
    }
    state
        .0
        .into_iter()
        .map(|b| b.into_iter().len())
        .sum::<usize>()
        .to_string()
}

pub fn task_1(file: &str) -> String {
    solve::<HashSet<_>>(file)
}

pub fn task_2(file: &str) -> String {
    solve::<Vec<_>>(file)
}
