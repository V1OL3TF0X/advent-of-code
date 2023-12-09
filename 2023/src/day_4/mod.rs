use regex::Regex;
use std::collections::HashSet;

pub fn task_1(file: &str) -> String {
    file.lines()
        .map(Card::from)
        .map(Card::get_score)
        .sum::<u32>()
        .to_string()
}

pub fn task_2(file: &str) -> String {
    let card_num = file.lines().count();
    let last_card_ind = card_num - 1;
    let mut card_count = vec![1; card_num];
    file.lines()
        .map(Card::from)
        .map(Card::into_match_num)
        .enumerate()
        .for_each(|(i, mut score)| {
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
    card_count.into_iter().sum::<u32>().to_string()
}

#[derive(Debug)]
struct Card {
    winning: HashSet<u32>,
    drawn: HashSet<u32>,
}

impl Card {
    fn get_score(self) -> u32 {
        match self.into_match_num() {
            0 => 0,
            found => 2u32.pow(found - 1),
        }
    }
    fn into_match_num(self) -> u32 {
        self.winning.intersection(&self.drawn).count() as u32
    }
}

lazy_static::lazy_static! {
    static ref REG: Regex = Regex::new(r"Card +(?:\d+): +(?<winning>\d+(?: +\d+)*) \| +(?<drawn>\d+(?: +\d+)*)").unwrap();
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let caps = REG.captures(value).expect(value);

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
