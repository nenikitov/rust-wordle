use std::{
    fmt::Display,
    iter
};

use rand::seq::SliceRandom;



#[derive(
    Debug,
    Clone, Copy,
    PartialEq, PartialOrd, Eq,
    Hash
)]
pub enum LetterScore {
    Unknown,
    Wrong,
    Present,
    Correct
}


#[derive(Debug)]
pub enum InvalidWord {
    DifferentLength,
    NotAWord
}

impl Display for InvalidWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DifferentLength => write!(f, "Input should be the same length as the word"),
            Self::NotAWord => write!(f, "This word is not in a dictionary"),
        }
    }
}



#[derive(Debug)]
pub struct WordleGame {
    words: Vec<String>,
    answer: String,
    lives: usize,
    guesses: [LetterScore; 26]
}

impl WordleGame {
    pub fn new_with_answer(words: Vec<String>, answer: &str) -> Self {
        if !words.contains(&answer.to_string()) {
            panic!("Word {answer} is not in the given word list");
        }

        Self {
            words,
            answer: answer.to_string(),
            lives: answer.len(),
            guesses: [LetterScore::Wrong; 26]
        }
    }

    pub fn new(words: Vec<String>) -> Self {
        let answer: String =
            if let Some(value) = words.choose(&mut rand::thread_rng()) {
                value.clone()
            }
            else {
                String::from("demo")
            };
        Self::new_with_answer(words, &answer)
    }

    pub fn guess(&mut self, guess: &str) -> Result<Vec<LetterScore>, InvalidWord> {
        if guess.len() != self.answer.len() {
            Err(InvalidWord::DifferentLength)
        }
        else if !self.words.contains(&guess.to_string()) {
            Err(InvalidWord::NotAWord)
        }
        else {
            let mut answer = self.answer.clone();
            // Initialize all wrong
            let mut score: Vec<LetterScore> =
                iter::repeat(LetterScore::Wrong)
                .take(guess.len())
                .collect();
            // Find the letter that are correct
            //for (letter_guess, letter_word) in guess.chars().zip(word.chars()) {}
            for i in 0..answer.len() {
                let char_answer = answer.chars().nth(i).unwrap();
                let char_guess = guess.chars().nth(i).unwrap();
                if char_guess == char_answer {
                    // Character matched, score and replace to not score again
                    score[i] = LetterScore::Correct;
                    answer = answer.replacen(char_guess, ":", 1);
                }
            }
            // Find the letters that are present
            for i in 0..answer.len() {
                let char_guess = guess.chars().nth(i).unwrap();
                if score[i] != LetterScore::Correct && answer.contains(char_guess) {
                    // Character matched, score and replace to not score again
                    score[i] = LetterScore::Present;
                    answer = answer.replacen(char_guess, ":", 1);
                }
            }
            self.lives -= 1;

            Ok(score)
        }
    }

    pub fn guess_empty(&self) -> Vec<LetterScore> {
        iter::repeat(LetterScore::Wrong)
            .take(self.answer.len())
            .collect()
    }


    pub fn known_guesses(&self, letters: &str) -> Vec<(char, LetterScore)> {
        letters
            .chars()
            .map(
                |c|
                (c, self.guess_at_index(c).to_owned())
            ).collect()
    }

    fn guess_at_index(&self, char: char) -> LetterScore {
        if !char.is_alphabetic() {
            panic!("Character {char} is not alphabetical")
        }

        self.guesses[char as usize - 'a' as usize]
    }
    fn set_guess_at_index(&mut self, char: char, score: LetterScore) {
        if !char.is_alphabetic() {
            panic!("Character {char} is not alphabetical")
        }

        if self.guess_at_index(char) < score {
            self.guesses[char as usize - 'a' as usize] = score;
        }
    }

    pub fn lives(&self) -> usize {
        self.lives
    }
}
