use std::{cmp::Reverse, collections::BinaryHeap, hash::Hash, str::FromStr};

use itertools::Itertools;
use ouroboros::self_referencing;
use rustc_hash::FxHashSet;
use union_find::{QuickFindUf, UnionBySize, UnionFind};

pub struct Solution;
const CONNECTIONS: usize = 10;
impl crate::task_fns::TaskFns for Solution {
    fn task_1(&self, file: &str) -> String {
        let mut grid = unsafe { Grid::from_str(file).unwrap_unchecked() };
        let mut uf = QuickFindUf::<UnionBySize>::new(grid.borrow_boxes().len());
        grid.with_distances_mut(|dist_heap| {
            for _ in 0..CONNECTIONS {
                let Reverse(pair) = unsafe { dist_heap.pop().unwrap_unchecked() };
                uf.union(pair.a, pair.b);
            }
        });
        let mut roots = FxHashSet::default();
        let mut res = 1;
        let mut circuits_by_size = BinaryHeap::from_iter((0..CONNECTIONS).flat_map(|i| {
            let root = uf.find(i);
            if roots.contains(&root) {
                return None;
            }
            roots.insert(root);
            Some(uf.get(i).size())
        }));
        for _ in 0..3 {
            res *= unsafe { circuits_by_size.pop().unwrap_unchecked() };
        }
        res.to_string()
    }

    fn task_2(&self, file: &str) -> String {
        let mut grid = unsafe { Grid::from_str(file).unwrap_unchecked() };
        let boxes_count = grid.borrow_boxes().len();
        let mut uf = QuickFindUf::<UnionBySize>::new(boxes_count);
        let last = grid.with_distances_mut(|dist_heap| loop {
            let Reverse(pair) = unsafe { dist_heap.pop().unwrap_unchecked() };
            uf.union(pair.a, pair.b);
            if (0..boxes_count)
                .map(|i| uf.find(i))
                .all_equal_value()
                .is_ok()
            {
                return pair;
            };
        });
        (last.boxes[last.a].x * last.boxes[last.b].x).to_string()
    }
}

#[self_referencing]
struct Grid {
    boxes: Vec<JunctionBox>,
    #[borrows(boxes)]
    #[covariant]
    distances: BinaryHeap<Reverse<JunctionBoxPair<'this>>>,
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let boxes: Vec<JunctionBox> = s.lines().map(JunctionBox::from_str).try_collect()?;
        Ok(GridBuilder {
            boxes,
            distances_builder: |boxes_ref| {
                (0..boxes_ref.len())
                    .combinations(2)
                    .map(|v| JunctionBoxPair {
                        a: v[0],
                        b: v[1],
                        boxes: boxes_ref,
                    })
                    .map(Reverse)
                    .collect::<BinaryHeap<_>>()
            },
        }
        .build())
    }
}

#[derive(Debug)]
struct JunctionBoxPair<'a> {
    a: usize,
    b: usize,
    boxes: &'a [JunctionBox],
}

impl JunctionBoxPair<'_> {
    fn dist(&self) -> usize {
        self.boxes[self.a].dist_sq(&self.boxes[self.b])
    }
}

impl Hash for JunctionBoxPair<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
        (self.boxes as *const [JunctionBox]).hash(state);
    }
}

impl Eq for JunctionBoxPair<'_> {}

impl PartialEq for JunctionBoxPair<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.boxes == other.boxes
            && ((self.a == other.a && self.b == other.b)
                || (self.a == other.b && self.b == other.a))
    }
}

impl Ord for JunctionBoxPair<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist().cmp(&other.dist())
    }
}

impl PartialOrd for JunctionBoxPair<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBox {
    fn dist_sq(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

impl FromStr for JunctionBox {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = s.split(',');
        let Some(x) = digits.next().and_then(|s| s.parse().ok()) else {
            return Err("couldn't parse x");
        };
        let Some(y) = digits.next().and_then(|s| s.parse().ok()) else {
            return Err("couldn't parse y");
        };
        let Some(z) = digits.next().and_then(|s| s.parse().ok()) else {
            return Err("couldn't parse x");
        };
        Ok(Self { x, y, z })
    }
}
