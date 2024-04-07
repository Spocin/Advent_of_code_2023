use std::fs;
use std::path::Path;

use indoc::indoc;

use crate::camel_cards::poker_hand::PokerHand;

mod poker_hand;

pub fn calculate_total_winnings(path_to_input: &Path) -> u64 {
    let input = fs::read_to_string(path_to_input);

    let mut cards = match input {
        Err(err) => panic!("Something wrong with the path! {:?}", err),
        Ok(input) => input
            .lines()
            .enumerate()
            .map(|(idx, line)| match PokerHand::new(line) {
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

    println!("Cards:");
    for card in &cards {
        println!("{:?}", card);
    }
    println!();

    cards.sort();

    println!("Cards:");
    for card in &cards {
        println!("{:?}", card);
    }

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