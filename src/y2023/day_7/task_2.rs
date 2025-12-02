use crate::vecmap::VecMap;

use super::{Card, Hand, MakePlayer};

pub(super) struct Task2;

impl MakePlayer for Task2 {
    fn make_card(c: char) -> Card {
        match c {
            'J' => Card::Joker,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            c => unreachable!("AKQT98765432J are only possible card values {c}"),
        }
    }

    fn hand(mut card_count: VecMap<Card, u8>) -> Hand {
        card_count.sort();
        let v: Vec<_> = card_count.into();
        let joker_count = v.iter().find(|(c, _)| c == &Card::Joker).map_or(0, |v| v.1);
        match (v.len(), joker_count, v[0].1) {
            (1, _, _) => Hand::FiveOfKind,              // XXXXX
            (2, 0, 4) => Hand::FourOfKind,              // XXXX Y=
            (2, 0, _) => Hand::FullHouse,               // XXX YY
            (2, _, _) => Hand::FiveOfKind,              // XXXX J or JJJJ X
            (3, 0, 3) => Hand::ThreeOfKind,             // XXX Y Z
            (3, _, 3) => Hand::FourOfKind,              // XXX J Y or JJJ X Y
            (3, 0, _) => Hand::TwoPair,                 // XX YY Z
            (3, 1, _) => Hand::FullHouse,               // XX YY J
            (3, 2, _) => Hand::FourOfKind,              // XX JJ Y
            (4, 2, _) | (4, 1, _) => Hand::ThreeOfKind, // JJ X Y Z or XX J Y Z
            (4, 0, _) | (5, 1, _) => Hand::Pair,        // XX Y Z A or X Y Z A J
            (5, _, _) => Hand::HighCard,
            v => unreachable!("those are the only patterns available {v:?}"),
        }
    }
}
