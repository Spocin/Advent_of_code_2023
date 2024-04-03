enum HandTypes {
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
    cards: [String; 5],
    hand_type: HandTypes
}