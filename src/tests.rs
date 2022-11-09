#[cfg(test)]
mod tests {
    use crate::wordle::{
        WordleGame,
        LetterScore
    };

    #[test]
    fn geese_test() {
        let mut game = WordleGame::new_with_answer(
            vec![
                String::from("those"),
                String::from("geese")
            ],
            "those"
        );

        assert_eq!(&[
            LetterScore::Wrong,
            LetterScore::Wrong,
            LetterScore::Wrong,
            LetterScore::Correct,
            LetterScore::Correct
            ],
            &game.guess(&String::from("geese")).unwrap()[..],
        );
    }

    #[test]
    fn added_test() {
        let mut game = WordleGame::new_with_answer(
            vec![
                String::from("dread"),
                String::from("added")
            ],
            "dread"
        );

        assert_eq!(&[
            LetterScore::Present,
            LetterScore::Present,
            LetterScore::Wrong,
            LetterScore::Present,
            LetterScore::Correct
            ],
            &game.guess(&String::from("added")).unwrap()[..],
        );
    }
}
