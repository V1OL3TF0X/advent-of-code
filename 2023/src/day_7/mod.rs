mod task_1;
mod task_2;

use crate::{utils::measure_elapsed, vecmap::VecMap};
pub fn task_1(file: &str) -> String {
    solve::<task_1::Task1>(file)
}
pub fn task_2(file: &str) -> String {
    solve::<task_2::Task2>(file)
}

fn solve<V: MakePlayer>(file: &str) -> String {
    measure_elapsed(|| {
        let mut hands: Vec<_> = file.lines().map(make_player::<V>).collect();
        hands.sort();
        hands
            .into_iter()
            .map(|h| h.bid)
            .enumerate()
            .map(|(i, b)| (i as u32 + 1) * b)
            .sum::<u32>()
            .to_string()
    })
}

#[derive(Eq, Debug)]
struct Player {
    hand: Hand,
    cards: Vec<Card>,
    bid: u32,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand
            .cmp(&other.hand)
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

fn make_player<V: MakePlayer>(value: &str) -> Player {
    let (hand, bid_str) = value.split_once(' ').unwrap();
    let (hand, cards) =
        hand.chars()
            .fold((VecMap::default(), vec![]), |(mut map, mut cards), card| {
                let card = V::make_card(card);
                cards.push(card);
                map.modify(card, |v| v + 1);
                (map, cards)
            });
    Player {
        hand: V::hand(hand),
        cards,
        bid: bid_str.parse().unwrap(),
    }
}

trait MakePlayer {
    fn make_card(c: char) -> Card;
    fn hand(card_count: VecMap<Card, u8>) -> Hand;
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub(super) enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub(super) enum Hand {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}
