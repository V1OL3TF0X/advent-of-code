use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

fn get_computer_connections(file: &str) -> FxHashMap<&str, FxHashSet<&str>> {
    file.lines()
        .map(|l| l.trim().split_once('-').expect(l))
        .fold(FxHashMap::default(), |mut hm, (from, to)| {
            hm.entry(from).or_default().insert(to);
            hm.entry(to).or_default().insert(from);
            hm
        })
}

fn max_clique<'a>(edges: &mut FxHashMap<&'a str, FxHashSet<&'a str>>) -> Vec<&'a str> {
    bron_kerbosh(
        FxHashSet::default(),
        edges.keys().cloned().collect(),
        FxHashSet::default(),
        edges,
    )
    .into_iter()
    .max_by(|a, b| a.len().cmp(&b.len()))
    .unwrap()
}

fn bron_kerbosh<'a>(
    current_clique: FxHashSet<&'a str>,
    mut candidates: FxHashSet<&'a str>,
    mut rejected: FxHashSet<&'a str>,
    edges: &mut FxHashMap<&'a str, FxHashSet<&'a str>>,
) -> FxHashSet<Vec<&'a str>> {
    let mut cliques = FxHashSet::default();
    if candidates.is_empty() && rejected.is_empty() {
        let mut current: Vec<_> = current_clique.iter().cloned().collect();
        current.sort_unstable();
        cliques.insert(current);
    }

    while let Some(c) = candidates.iter().next().cloned() {
        let mut new_current = current_clique.clone();
        new_current.insert(c);
        let new_rejected = rejected
            .intersection(edges.get(c).unwrap())
            .cloned()
            .collect();
        let new_candidates = candidates
            .intersection(edges.get(c).unwrap())
            .cloned()
            .collect();
        cliques.extend(bron_kerbosh(
            new_current,
            new_candidates,
            new_rejected,
            edges,
        ));
        rejected.insert(c);
        candidates.remove(c);
    }
    cliques
}

pub struct Solution;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        let mut three_interconnected = FxHashSet::default();
        let connections = get_computer_connections(file);
        connections.iter().for_each(|(from, to_list)| {
            to_list.iter().tuple_combinations().for_each(|(a, b)| {
                if connections.get(a).is_some_and(|a_list| a_list.contains(b)) {
                    let mut key = [from, a, b];
                    key.sort_unstable();
                    three_interconnected.insert(key);
                }
            });
        });

        three_interconnected
            .into_iter()
            .filter(|k| k.iter().any(|c| c.starts_with('t')))
            .count()
            .to_string()
    }

    fn task_2(&self, file: &str) -> String {
        max_clique(&mut get_computer_connections(file)).join(",")
    }
}
