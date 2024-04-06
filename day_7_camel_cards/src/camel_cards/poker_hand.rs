use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowedCardLabel {
    A = 13,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandTypes {
    FiveOfAKind = 7,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, Ord)]
pub struct PokerHand {
    pub bid: u16,
    pub cards: [AllowedCardLabel; 5],
    pub hand_type: HandTypes
}

impl PokerHand {
    pub fn new(line: &str) -> PokerHand {
        // TODO Move parsing line here
        // TODO Match char from line to AllowedCardLabel

        PokerHand {
            bid,
            cards,
            hand_type: Self::compute_hand_type_from_cards(&cards)
        }
    }

    fn compute_hand_type_from_cards(cards: &[AllowedCardLabel; 5]) -> HandTypes {
        //Five of a kind
        if cards.iter().all(|&el| el == cards[0]) {
            return HandTypes::FiveOfAKind;
        }
        
        //Four of a kind
        for i in 0..3 {
            if cards.iter().filter(|&el| *el == cards[i]).count() == 4 {
                return HandTypes::FourOfAKind;
            }
        }

        let has_triplet = Self::check_has_triplet(cards);
        let pairs_count = Self::count_pairs(cards);

        //Full house
        if has_triplet && pairs_count == 1 {
            return HandTypes::FullHouse;
        }

        //Three of a kind
        if has_triplet {
            return HandTypes::ThreeOfAKind;
        }

        //Two pair
        if pairs_count == 2 {
            return HandTypes::TwoPair;
        }

        //One pair
        if pairs_count == 1 {
            return HandTypes::OnePair;
        }
        
        return HandTypes::HighCard;
    }

    fn check_has_triplet(cards: &[AllowedCardLabel; 5]) -> bool {
        for i in 0..3 {
            if cards.iter().filter(|&el| *el == cards[i]).count() == 3 {
                return true;
            }
        }

        return false;
    }

    fn count_pairs(cards: &[AllowedCardLabel; 5]) -> u8 {
        let mut count = 0u8;
        let mut cards = cards.clone().to_vec();

        while cards.len() != 0 {
            let card_occurrence = cards.iter()
                .filter(|&el| *el == cards[0])
                .count();

            if card_occurrence == 2 {
                count += 1;
            }

            let tmp = cards[0];
            cards.retain(|&el| el != tmp);
        }

        return count;
    }
}

/*impl PartialEq<Self> for PokerHand {
    fn eq(&self, other: &Self) -> bool {
        if self.bid != other.bid {
            return false;
        }

        for i in 0..self.cards.len() {
            if self.cards[i] != other.cards[i] {
                return false;
            }
        }

        return true;
    }
}*/

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type < other.hand_type {
            return Some(Ordering::Less);
        }

        if self.hand_type > other.hand_type {
            return Some(Ordering::Greater);
        }

        // Sequentially compare cards
        let cards_iter = self.cards
            .iter()
            .zip(&other.cards)
            .find_map(|(x,y)| {
                if x < y { return Some(Ordering::Less); }
                if x > y { return Some(Ordering::Greater); }
                return None;
            });

        return match cards_iter {
            Some(val) => Some(val),
            None => Some(Ordering::Equal),
        }
    }
}

#[cfg(test)]
mod compute_hand_type_from_cards {
    use crate::camel_cards::poker_hand::{HandTypes, PokerHand};

    #[test]
    fn five_of_a_kind() {
        let cards = ['A','A','A','A','A'];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::FiveOfAKind);
    }

    #[test]
    fn four_of_a_kind() {
        let cards = ['A','A','8','A','A'];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::FourOfAKind);
    }

    #[test]
    fn full_house() {
        let cards = ['2','3','3','3','2'];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::FullHouse);
    }

    #[test]
    fn three_of_a_kind() {
        let cards = ['T','T','T','9','8'];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::ThreeOfAKind);
    }

    #[test]
    fn two_pair() {
        let cards = ['2','3','4','3','2'];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::TwoPair);
    }

    #[test]
    fn one_pair() {
        let cards = ['A','2','3','A','4'];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::OnePair);
    }

    #[test]
    fn high_card() {
        let cards = ['2','3','4','5','6'];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::HighCard);
    }
}