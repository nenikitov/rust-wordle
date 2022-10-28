use rand::seq::SliceRandom;

enum LetterScore {
    Wrong,
    Present,
    Correct
}


#[derive(Debug)]
pub struct WordleGame {
    word: String
}

impl WordleGame {
    pub fn new(words: &Vec<String>) -> Self {
        let word: String =
            if let Some(value) = words.choose(&mut rand::thread_rng()) {
                value.clone()
            }
            else {
                String::from("demo")
            };
        Self { word }
    }
}
