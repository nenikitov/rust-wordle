use rand::seq::SliceRandom;

use crate::words::count_all;

#[derive(Debug)]
pub enum LetterScore {
    Wrong,
    Present,
    Correct
}

#[derive(Debug)]
pub enum InvalidWord {
    DifferentLength,
    NotAWord
}


#[derive(Debug)]
pub struct WordleGame {
    words: Vec<String>,
    word: String
}

impl WordleGame {
    pub fn new(words: Vec<String>) -> Self {
        let word: String =
            if let Some(value) = words.choose(&mut rand::thread_rng()) {
                value.clone()
            }
            else {
                String::from("demo")
            };
        Self { words, word }
    }

    pub fn guess(&self, guess: &String) -> Result<Vec<LetterScore>, InvalidWord> {
        if guess.len() != self.word.len() {
            Err(InvalidWord::DifferentLength)
        }
        else if !self.words.contains(guess) {
            println!("{:?}", self.words);
            Err(InvalidWord::NotAWord)
        }
        else {
            Ok(
                guess
                    .char_indices()
                    .map(|(i, _)| self.guess_for_current_char(i, guess))
                    .collect()
            )
        }
    }

    fn guess_for_current_char(&self, i: usize, guess: &String) -> LetterScore {
        let char_guess = guess.chars().nth(i).unwrap();
        let char_word = self.word.chars().nth(i).unwrap();

        let previous_guesses = String::from(&guess[..=i]);
        let char_guess_set = String::from(char_guess);

        let guesses_in_word = count_all(&self.word, &char_guess_set);
        let guesses_in_previous_guess = count_all(&previous_guesses, &char_guess_set);

        if char_guess == char_word {
            // Same character on the same position - correct
            LetterScore::Correct
        }
        else if guesses_in_word != 0 && guesses_in_previous_guess <= guesses_in_word {
            // 
            LetterScore::Present
        }
        else {
            // Other checks failed - wrong
            LetterScore::Wrong
        }
    }
}
