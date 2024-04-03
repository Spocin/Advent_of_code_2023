pub enum HandTypes {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

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
        // TODO Compute HandType
        return HandTypes::FiveOfAKind
    }
}