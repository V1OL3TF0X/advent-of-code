use crate::vecmap::VecMap;

use super::{Card, Hand, MakePlayer};

pub(super) struct Task1;

impl MakePlayer for Task1 {
    fn make_card(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            c => unreachable!("AKQT98765432J are only possible card values {c}"),
        }
    }

    fn hand(mut card_count: VecMap<Card, u8>) -> Hand {
        card_count.sort();
        let v: Vec<_> = card_count.into();
        match (v.len(), v[0].1) {
            (1, _) => Hand::FiveOfKind,  // XXXXX
            (2, 4) => Hand::FourOfKind,  // XXXX Y
            (2, _) => Hand::FullHouse,   // XXX YY
            (3, 3) => Hand::ThreeOfKind, // XXX Y z
            (3, _) => Hand::TwoPair,     // XX YY Z
            (4, _) => Hand::Pair,        // XX Y Z A
            (5, _) => Hand::HighCard,    // X Y Z A B
            _ => unreachable!("hand has 5 cards!!!"),
        }
    }
}
