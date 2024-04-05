#[derive(Debug)]
#[derive(PartialEq)]
pub enum HandTypes {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug)]
pub struct PokerHand {
    pub bid: u16,
    pub cards: [char; 5],
    pub hand_type: HandTypes
}

impl PokerHand {
    pub fn new(bid: u16, cards: [char; 5]) -> PokerHand {
        PokerHand {
            bid,
            cards,
            hand_type: Self::compute_hand_type_from_cards(&cards)
        }
    }

    fn compute_hand_type_from_cards(cards: &[char; 5]) -> HandTypes {
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

    fn check_has_triplet(cards: &[char; 5]) -> bool {
        for i in 0..3 {
            if cards.iter().filter(|&el| *el == cards[i]).count() == 3 {
                return true;
            }
        }

        return false;
    }

    fn count_pairs(cards: &[char; 5]) -> u8 {
        let mut count = 0u8;
        let mut cards: Vec<char> = cards.clone().to_vec();

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

impl PartialEq for PokerHand {
    fn eq(&self, other: &Self) -> bool {
        if self.bid != other.bid {
            return false;
        }

        if self.cards != other.cards {
            return false;
        }

        return true;
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