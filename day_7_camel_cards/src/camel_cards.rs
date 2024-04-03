use std::fs;
use std::path::Path;

use crate::camel_cards::poker_hand::PokerHand;

mod poker_hand;

const ALLOWED_CARD_LABELS: [char; 13] = [
    'A',
    'K',
    'Q',
    'J',
    'T',
    '9',
    '8',
    '7',
    '6',
    '5',
    '4',
    '3',
    '2',
];

pub fn calculate_total_winnings(path_to_input: &Path) -> u64 {
    let input = fs::read_to_string(path_to_input);

    let cards = match input {
        Err(err) => panic!("Something wrong with the path! {:?}", err),
        Ok(input) => input
            .lines()
            .enumerate()
            .map(|(idx, line)| match parse_line_into_card(line) {
                Err(line_that_errored) => panic!(
                    "Could not parse line at: {} \
                     Error: {} \
                     Line: {}
                    "
                , idx, line_that_errored.1, line_that_errored.0),
                Ok(val) => val,
            })
            .collect::<Vec<&PokerHand>>()
    };

    // TODO Sort cards

    return cards
        .iter()
        .enumerate()
        .map(|(idx, hand)| {
            let idx_casted = u16::try_from(idx);
            match idx_casted {
                Err(err) => panic!("Error while casting idx to u16 {:?}", err),
                Ok(val) => hand.bid * (val + 1)
            }
        })
        .fold(0u64, |acc, el|
            match acc.checked_add(u64::from(el)) {
                None => panic!("Overflow occurred while folding bids!"),
                Some(val) => val,
            }
        )
}

fn parse_line_into_card(line: &str) -> Result<&PokerHand, (&str, &str)> {
    let line_split = match line.split_once(" ") {
        None => return Err((line, "No space to split by")),
        Some(val) => val
    };

    if line_split.0.len() != 5 {
        return Err((line, "Hand must have exactly 5 cards"));
    }
    
    if !line_split.0.chars().all(|el| ALLOWED_CARD_LABELS.contains(&el)) {
        let error = format!("Invalid card label. Must be one of: {:?}", ALLOWED_CARD_LABELS).as_str();
        return Err((line, error))
    }

    let cards_parsed: [char; 5] = line_split.0
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .unwrap();

    let bid_parsed = match line_split.1.parse::<u16>() {
        Err(err) => {
            let error = format!("Could not parse bid: {} to u16. {}", line_split.1, err).as_str();
            return Err((line, error))
        },
        Ok(val) => val
    };

    return Ok(&PokerHand::new(
        bid_parsed,
        cards_parsed
    ));
}