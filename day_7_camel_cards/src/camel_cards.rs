use std::fs;
use std::path::Path;

use indoc::indoc;

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
                    indoc! {r#"
                    Could not parse line at: {}
                    Error: "{}"
                    Line: {}
                    "#}
                , idx, line_that_errored.1, line_that_errored.0),
                Ok(val) => val,
            })
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
                Ok(val) => hand.bid * (val + 1)
            }
        })
        .fold(0u64, |acc, el|
            /*
             We would have to fold max u16 value 281479271743489 times to overflow u64.
             Nevertheless. Good practice is good practice :)

             I won't make a test for it. Too much memory I guess (~500MB).
            */
            match acc.checked_add(u64::from(el)) {
                None => panic!("Overflow occurred while folding bids!"),
                Some(val) => val,
            }
        )
}

fn parse_line_into_card(line: &str) -> Result<PokerHand, (&str, String)> {
    /*  FIXME
    *    Should this logic be inside PokerHand struct?
    *    Having outside of the struct allows call for new with invalid parameters.
    */
    let line_split = match line.split_once(" ") {
        None => return Err((line, "No space to split by".into())),
        Some(val) => val
    };

    if line_split.0.len() != 5 {
        return Err((line, "Hand must have exactly 5 cards".into()));
    }
    
    if !line_split.0.chars().all(|el| ALLOWED_CARD_LABELS.contains(&el)) {
        let error = format!("Invalid card label. Must be one of: {:?}", ALLOWED_CARD_LABELS);
        return Err((line, error))
    }

    let cards_parsed: [char; 5] = line_split.0
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .unwrap();

    let bid_parsed = match line_split.1.parse::<u16>() {
        Err(err) => {
            let error = format!("Could not parse bid: {} to u16. {}", line_split.1, err);
            return Err((line, error))
        },
        Ok(val) => val
    };

    return Ok(PokerHand::new(
        bid_parsed,
        cards_parsed
    ));
}

#[cfg(test)]
mod calculate_total_winnings {
    use std::fs::{File, remove_file};
    use std::io::Write;
    use std::path::Path;

    use crate::camel_cards::calculate_total_winnings;

    struct TestFile {
        path: &'static Path,
    }

    impl TestFile {
        fn new(path: &'static str) -> TestFile {
            TestFile { path: Path::new(path) }
        }
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            remove_file(self.path).unwrap()
        }
    }

    #[test]
    #[should_panic(expected = "Something wrong with the path!")]
    fn it_should_panic_when_path_it_invalid() {
        let invalid_path = Path::new("fwafagwagagawa");

        calculate_total_winnings(invalid_path);
    }

    #[test]
    #[should_panic(expected = "Could not parse line at: 0\n\
                               Error: \"No space to split by\"\n\
                               Line: InvalidData"
    )]
    fn it_should_panic_when_parsing_card_fails() {
        let test_file = TestFile::new("../day_7_camel_cards/resources/test_input_mock.txt");
        let mut file = File::create(&test_file.path).unwrap();

        writeln!(file, "InvalidData").unwrap();

        calculate_total_winnings(test_file.path);
    }
}

#[cfg(test)]
mod parse_line_into_card {
    use crate::camel_cards::{ALLOWED_CARD_LABELS, parse_line_into_card};
    use crate::camel_cards::poker_hand::PokerHand;

    #[test]
    fn it_should_return_err_when_line_has_no_space() {
        let mock_line = "QT9KA116";

        let result = parse_line_into_card(mock_line);

        let message = "No space to split by";
        assert_eq!(result.err().unwrap(), (mock_line, message.into()))
    }

    #[test]
    fn it_should_return_err_when_hand_does_not_have_5_cards() {
        let mock_line = "QT9K 116";

        let result = parse_line_into_card(mock_line);

        let message = "Hand must have exactly 5 cards";
        assert_eq!(result.err().unwrap(), (mock_line, message.into()))
    }

    #[test]
    fn it_should_return_err_when_hand_uses_not_allowed_char() {
        let mock_line = "QT9KZ 116";

        let result = parse_line_into_card(mock_line);

        let message = format!("Invalid card label. Must be one of: {:?}", ALLOWED_CARD_LABELS);
        assert_eq!(result.err().unwrap(), (mock_line, message));
    }

    #[test]
    fn it_should_return_err_when_bid_wont_fit_in_u16() {
        let mock_line = "QT9KK 65536";

        let result = parse_line_into_card(mock_line);

        let message = "Could not parse bid: 65536 to u16. number too large to fit in target type";
        assert_eq!(result.err().unwrap(), (mock_line, message.into()));
    }

    #[test]
    fn it_should_return_hand() {
        let mock_line = "QT9KK 65535";

        let result = parse_line_into_card(mock_line).unwrap();

        let expected_hand = PokerHand::new(65535, ['Q', 'T', '9', 'K', 'K']);
        assert_eq!(result, expected_hand);
    }
}