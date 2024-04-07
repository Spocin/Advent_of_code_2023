use std::cmp::Ordering;
use std::convert::Into;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowedCardLabel {
    A = 13,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
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
    pub bid: u32,
    pub cards: [AllowedCardLabel; 5],
    pub hand_type: HandTypes,
}

const NO_SPACE_ERR: &str = "No space to split by";
const INVALID_LENGTH_ERR: &str = "Hand must have exactly 5 cards";
const INVALID_CARD_LABEL_ERR: &str = "Invalid card label";

impl PokerHand {
    pub fn new(line: &str) -> Result<PokerHand, (&str, String)> {
        let line_split = match line.split_once(" ") {
            None => return Err((line, NO_SPACE_ERR.into())),
            Some(val) => val
        };

        if line_split.0.len() != 5 {
            return Err((line, INVALID_LENGTH_ERR.into()));
        }

        let cards_parsed: [AllowedCardLabel; 5] = match Self::parse_card_label_to_enum_values(line_split.0) {
            Ok(val) => val.try_into().unwrap(),
            Err(err) => return Err((line, err)),
        };

        let bid_parsed = match line_split.1.parse::<u32>() {
            Err(err) => {
                let error = format!("Could not parse bid: {} to u32. {}", line_split.1, err);
                return Err((line, error));
            }
            Ok(val) => val
        };

        return Ok(PokerHand {
            bid: bid_parsed,
            cards: cards_parsed.clone(),
            hand_type: Self::compute_hand_type_from_cards(&cards_parsed),
        });
    }

    fn compute_hand_type_from_cards(cards: &[AllowedCardLabel; 5]) -> HandTypes {
        //Five of a kind
        if cards.iter().all(|el| *el == cards[0] || *el == AllowedCardLabel::J) {
            return HandTypes::FiveOfAKind;
        }

        //Four of a kind
        for i in 0..3 {
            let cards_count = cards
                .iter()
                .filter(|&el| *el == cards[i] || *el == AllowedCardLabel::J)
                .count();

            if cards_count == 4 {
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
            let cards_count = cards
                .iter()
                .filter(|&el| *el == cards[i] || *el == AllowedCardLabel::J)
                .count();

            if cards_count == 3 {
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
                .filter(|&el| *el == cards[0] || *el == AllowedCardLabel::J)
                .count();

            if card_occurrence == 2 {
                count += 1;
            }

            let tmp = cards[0].clone();
            cards.retain(|el| *el != tmp);
        }

        return count;
    }

    fn parse_card_label_to_enum_values(cards: &str) -> Result<Vec<AllowedCardLabel>, String> {
        return cards
            .chars()
            .map(|el| match el {
                'A' => Ok(AllowedCardLabel::A),
                'K' => Ok(AllowedCardLabel::K),
                'Q' => Ok(AllowedCardLabel::Q),
                'J' => Ok(AllowedCardLabel::J),
                'T' => Ok(AllowedCardLabel::T),
                '9' => Ok(AllowedCardLabel::Nine),
                '8' => Ok(AllowedCardLabel::Eight),
                '7' => Ok(AllowedCardLabel::Seven),
                '6' => Ok(AllowedCardLabel::Six),
                '5' => Ok(AllowedCardLabel::Five),
                '4' => Ok(AllowedCardLabel::Four),
                '3' => Ok(AllowedCardLabel::Three),
                '2' => Ok(AllowedCardLabel::Two),
                _ => Err(INVALID_CARD_LABEL_ERR.into())
            })
            .collect::<Result<Vec<AllowedCardLabel>, String>>();
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type > other.hand_type {
            return Some(Ordering::Less);
        }

        if self.hand_type < other.hand_type {
            return Some(Ordering::Greater);
        }

        // Sequentially compare cards
        let cards_iter = self.cards
            .iter()
            .zip(&other.cards)
            .find_map(|(x, y)| {
                if x < y { return Some(Ordering::Greater); }
                if x > y { return Some(Ordering::Less); }
                return None;
            });

        return match cards_iter {
            Some(val) => Some(val),
            None => Some(Ordering::Equal),
        };
    }
}

#[cfg(test)]
mod poker_hand_new {
    use crate::camel_cards::poker_hand::{AllowedCardLabel, INVALID_CARD_LABEL_ERR, INVALID_LENGTH_ERR, NO_SPACE_ERR, PokerHand};

    #[test]
    fn it_should_return_err_when_line_has_no_space() {
        let mock_line = "QT9KA116";

        let result = PokerHand::new(mock_line);

        assert_eq!(result.err().unwrap(), (mock_line, NO_SPACE_ERR.into()))
    }

    #[test]
    fn it_should_return_err_when_hand_does_not_have_5_cards() {
        let mock_line = "QT9K 116";

        let result = PokerHand::new(mock_line);

        assert_eq!(result.err().unwrap(), (mock_line, INVALID_LENGTH_ERR.into()))
    }

    #[test]
    fn it_should_return_err_when_hand_uses_not_allowed_char() {
        let mock_line = "QT9KZ 116";

        let result = PokerHand::new(mock_line);

        assert_eq!(result.err().unwrap(), (mock_line, INVALID_CARD_LABEL_ERR.into()));
    }

    #[test]
    fn it_should_return_err_when_bid_wont_fit_in_u16() {
        let mock_line = "QT9KK 4294967296";

        let result = PokerHand::new(mock_line);

        let message = "Could not parse bid: 4294967296 to u32. number too large to fit in target type";
        assert_eq!(result.err().unwrap(), (mock_line, message.into()));
    }

    #[test]
    fn it_should_return_correct_hand_bid() {
        let mock_line = "QT9KK 65535";

        let result = PokerHand::new(mock_line);

        assert_eq!(result.unwrap().bid, 65535);
    }

    #[test]
    fn it_should_return_correct_cards() {
        let mock_line = "QT9KK 65535";

        let result = PokerHand::new(mock_line);

        assert_eq!(result.unwrap().cards, [
            AllowedCardLabel::Q,
            AllowedCardLabel::T,
            AllowedCardLabel::Nine,
            AllowedCardLabel::K,
            AllowedCardLabel::K
        ])
    }
}

#[cfg(test)]
mod compute_hand_type_from_cards {
    use crate::camel_cards::poker_hand::{AllowedCardLabel, HandTypes, PokerHand};

    #[test]
    fn five_of_a_kind() {
        let cards = [
            AllowedCardLabel::A,
            AllowedCardLabel::A,
            AllowedCardLabel::A,
            AllowedCardLabel::A,
            AllowedCardLabel::A,
        ];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::FiveOfAKind);
    }

    #[test]
    fn four_of_a_kind() {
        let cards = [
            AllowedCardLabel::A,
            AllowedCardLabel::A,
            AllowedCardLabel::Eight,
            AllowedCardLabel::A,
            AllowedCardLabel::A,
        ];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::FourOfAKind);
    }

    #[test]
    fn full_house() {
        let cards = [
            AllowedCardLabel::Two,
            AllowedCardLabel::Three,
            AllowedCardLabel::Three,
            AllowedCardLabel::Three,
            AllowedCardLabel::Two,
        ];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::FullHouse);
    }

    #[test]
    fn three_of_a_kind() {
        let cards = [
            AllowedCardLabel::T,
            AllowedCardLabel::T,
            AllowedCardLabel::T,
            AllowedCardLabel::Nine,
            AllowedCardLabel::Eight,
        ];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::ThreeOfAKind);
    }

    #[test]
    fn two_pair() {
        let cards = [
            AllowedCardLabel::Two,
            AllowedCardLabel::Three,
            AllowedCardLabel::Four,
            AllowedCardLabel::Three,
            AllowedCardLabel::Two,
        ];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::TwoPair);
    }

    #[test]
    fn one_pair() {
        let cards = [
            AllowedCardLabel::A,
            AllowedCardLabel::Two,
            AllowedCardLabel::Three,
            AllowedCardLabel::A,
            AllowedCardLabel::Four,
        ];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::OnePair);
    }

    #[test]
    fn high_card() {
        let cards = [
            AllowedCardLabel::Two,
            AllowedCardLabel::Three,
            AllowedCardLabel::Four,
            AllowedCardLabel::Five,
            AllowedCardLabel::Six,
        ];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::HighCard);
    }

    #[test]
    fn four_of_a_kind_with_joker() {
        let cards = [
          AllowedCardLabel::Q,
          AllowedCardLabel::J,
          AllowedCardLabel::J,
          AllowedCardLabel::Q,
          AllowedCardLabel::Two,
        ];

        let hand_type = PokerHand::compute_hand_type_from_cards(&cards);

        assert_eq!(hand_type, HandTypes::FourOfAKind);
    }
}