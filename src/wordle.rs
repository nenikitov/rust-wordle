use std::{fmt::Display, iter::repeat};

use rand::seq::SliceRandom;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
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
    answer: String
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
        Self { words, answer: word }
    }

    pub fn new_with_answer(words: Vec<String>, answer: String) -> Self {
        if !words.contains(&answer) {
            panic!("Word list should contain the target word");
        }
        else {
            Self { words, answer }
        }
    }

    pub fn guess(&self, guess: &String) -> Result<Vec<LetterScore>, InvalidWord> {
        if guess.len() != self.answer.len() {
            Err(InvalidWord::DifferentLength)
        }
        else if !self.words.contains(guess) {
            Err(InvalidWord::NotAWord)
        }
        else {
            let mut answer = self.answer.clone();
            // Initialize all wrong
            let mut score: Vec<LetterScore> = repeat(LetterScore::Wrong).take(guess.len()).collect();
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

            Ok(score)
        }
    }

    pub fn guess_empty(&self) -> Vec<LetterScore> {
        self.answer
            .chars()
            .map(|_| LetterScore::Wrong)
            .collect()
    }
}
