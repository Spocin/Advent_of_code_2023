use std::fs;
use std::path::Path;

use crate::camel_cards::poker_hand::PokerHand;

mod poker_hand;

pub fn calculate_total_winnings(path_to_input: &Path) -> u64 {
    let input = fs::read_to_string(path_to_input);

    let cards = match input {
        Err(err) => panic!("Something wrong with the path! {:?}", err),
        Ok(input) => input
            .lines()
            .map(|line| parse_line_into_card(line))
            .collect::<Vec<PokerHand>>()
    };

    // TODO Sort cards

    return cards
        .iter()
        .enumerate()
        .map(|(idx, hand)| {
            let idx_casted = u16::try_from(idx);
            match idx_casted {
                Err(err) => panic!("Error while casting idx to u16 {:?}", err),
                Ok(val) => return hand.bid * (val + 1)
            }
        })
        .fold(0u64, |acc, el|
            match acc.checked_add(u64::from(el)) {
                None => panic!("Overflow occurred while folding bids!"),
                Some(val) => return val,
            }
        )
}

fn parse_line_into_card(line: &str) -> PokerHand {

}