use itertools::Itertools;
use rustc_hash::FxHashMap;
use union_find::{QuickUnionUf, UnionBySize, UnionFind};

pub fn task_1(file: &str) -> String {
    let mut table: Vec<Vec<char>> = file.lines().map(|l| l.chars().collect()).collect();
    let mut perimiters = vec![4; table[0].len() * table.len()];
    let mut uf = QuickUnionUf::<UnionBySize>::new(table.len() * table[0].len());
    let mut i = 0;
    table.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, c)| {
            if x < table[0].len() - 1 && *c == line[x + 1] {
                let a = i;
                let b = i + 1;
                let rep_a = uf.find(a);
                let rep_b = uf.find(b);
                if uf.union(a, b) {
                    perimiters[rep_a] += perimiters[rep_b];
                }
                perimiters[rep_a] -= 2;
                perimiters[rep_b] = perimiters[rep_a];
            }
            if y < table.len() - 1 && *c == table[y + 1][x] {
                let a = i;
                let b = i + table[0].len();
                let rep_a = uf.find(a);
                let rep_b = uf.find(b);
                if uf.union(a, b) {
                    perimiters[rep_a] += perimiters[rep_b];
                }
                perimiters[rep_a] -= 2;
                perimiters[rep_b] = perimiters[rep_a];
            }
            i += 1;
        });
    });
    i = 0;
    (0..table.len()).for_each(|y| {
        (0..table[0].len()).for_each(|x| {
            let p = uf.find(i);
            if p == i {
                println!("{}->{} : {}", table[y][x], uf.get(p).size(), perimiters[p]);
            }
            i += 1;
        });
    });
    (0..i)
        .filter_map(|r| (uf.find(r) == r).then(|| uf.get(r).size() * perimiters[r]))
        .sum::<usize>()
        .to_string()
}

const DIRECTIONS: [isize; 2] = [-1, 1];

pub fn task_2(file: &str) -> String {
    let mut table: Vec<Vec<char>> = file
        .lines()
        .map(|l| {
            let mut symbols = vec!['.'];
            symbols.extend(l.chars());
            symbols.push('.');
            symbols
        })
        .collect();
    table.push(vec!['.'; table[0].len()]);
    table.insert(0, vec!['.'; table[0].len()]);
    let table_width = table[0].len() - 2;
    let table_height = table[1].len() - 2;
    let table_size = table_width * table_height;
    let mut corners = vec![0; table_size];
    let mut uf = QuickUnionUf::<UnionBySize>::new(table_size);
    let mut i = 0;
    table
        .iter()
        .enumerate()
        .skip(1)
        .take(table_height)
        .for_each(|(y, line)| {
            line.iter()
                .enumerate()
                .skip(1)
                .take(table_width)
                .for_each(|(x, c)| {
                    corners[i] = (0..=1)
                        .cartesian_product(0..=1)
                        .filter(|dirs| {
                            let check_y = (y as isize + DIRECTIONS[dirs.0]) as usize;
                            let check_x = (x as isize + DIRECTIONS[dirs.1]) as usize;
                            let one_of = table[check_y][x] == *c;
                            one_of == (table[y][check_x] == *c)
                                && if one_of {
                                    table[check_y][check_x] != *c
                                } else {
                                    true
                                }
                        })
                        .count();
                    println!(
                        "{x} {y} ({el}) has {c} corners",
                        el = table[y][x],
                        c = corners[i]
                    );
                    if x < table_width && *c == table[y][x + 1] {
                        uf.union(i, i + 1);
                    }
                    if y < table_width && *c == table[y + 1][x] {
                        let a = i;
                        let b = i + table_width;
                        uf.union(a, b);
                    }
                    i += 1;
                });
        });
    let mut total_corners = FxHashMap::default();
    (0..table_size).filter(|i| corners[*i] != 0).for_each(|i| {
        let p = uf.find(i);
        total_corners
            .entry(p)
            .and_modify(|c| *c += corners[i])
            .or_insert(corners[i]);
    });
    i = 0;
    (1..=table_height).for_each(|y| {
        (1..=table_width).for_each(|x| {
            let p = uf.find(i);
            if p == i {
                println!(
                    "{}({}, {})->{} : {}",
                    table[y][x],
                    x,
                    y,
                    uf.get(p).size(),
                    total_corners.get(&p).unwrap()
                );
            }
            i += 1;
        });
    });

    (0..i)
        .filter_map(|r| {
            (uf.find(r) == r).then(|| uf.get(r).size() * total_corners.get(&r).unwrap())
        })
        .sum::<usize>()
        .to_string()
}
