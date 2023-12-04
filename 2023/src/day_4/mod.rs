use std::collections::HashSet;

use regex::Regex;
pub fn task_1(file: &str) -> String {
    let sum: u32 = file.lines().map(|l| Card::from(l).get_score()).sum();
    sum.to_string()
}

pub fn task_2(file: &str) -> String {
    let cards: Vec<_> = file.lines().map(Card::from).collect();
    let mut card_count = vec![1; cards.len()];
    let last_card_ind = card_count.len() - 1;
    cards.iter().enumerate().for_each(|(i, c)| {
        let mut score = c.get_match_num();
        if score == 0 {
            return;
        }
        let mut add_to = i + 1;
        while add_to < last_card_ind && score != 0 {
            card_count[add_to] += card_count[i];
            add_to += 1;
            score -= 1;
        }
        if score != 0 {
            card_count[last_card_ind] += card_count[i] * score;
        }
    });
    card_count.iter().sum::<u32>().to_string()
}

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    drawn: HashSet<u32>,
}

impl Card {
    fn get_score(&self) -> u32 {
        match self.get_match_num() {
            0 => 0,
            found => 2u32.pow(found - 1),
        }
    }
    fn get_match_num(&self) -> u32 {
        self.drawn.intersection(&self.winning).count() as u32
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let caps =
            Regex::new(r"Card +(?:\d+): +(?<winning>\d+(?: +\d+)*) \| +(?<drawn>\d+(?: +\d+)*)")
                .unwrap()
                .captures(value)
                .expect(value);

        Self {
            winning: to_nums(&caps["winning"]),
            drawn: to_nums(&caps["drawn"]),
        }
    }
}

fn to_nums(num_str: &str) -> HashSet<u32> {
    num_str
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}
