use std::{
    fs::File,
    io::{
        BufReader,
        BufRead,
        self
    }, fmt::{Display} ,ops::RangeInclusive
};



type Words = Vec<String>;
type WordErrors = Vec<WordError>;
type InvalidWords = Vec<InvalidWord>;

#[derive(Debug)]
pub struct InvalidWord {
    pub pos: usize,
    pub word: String,
    pub errors: WordErrors
}

impl Display for InvalidWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "- '{}' at index {}:\n{}",
            self.word,
            self.pos,
            self.errors
                .iter()
                .map(
                    |e|
                    format!("    - {}", e)
                ).collect::<Words>()
                .join("\n")
        )
    }
}


#[derive(Debug)]
pub enum WordListError {
    NoFile,
    Empty,
    InvalidWords {
        words: InvalidWords
    }
}

impl Display for WordListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFile => write!(f, "File cannot be read/does not exist"),
            Self::Empty => write!(f, "Word list format is improperly formatted"),
            Self::InvalidWords { words } => write!(f, "Some words have errors:\n{}", words.iter().map(|w| w.to_string()).collect::<Words>().join("\n"))
        }
    }
}



#[derive(Debug)]
pub enum WordError {
    InvalidCharacter {
        pos: usize,
        char: char
    },
    InvalidLength {
        len: usize
    },
}

impl Display for WordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WordError::InvalidCharacter {pos, char} => write!(f, "Invalid character '{char}' at index {pos}"),
            WordError::InvalidLength {len} => write!(
                f, "Length of {len}, it should be between {} and {}",
                WORD_RANGE.min().unwrap(),
                WORD_RANGE.max().unwrap()
            )
        }
    }
}



pub fn default_words() -> Words {
    include_str!("../res/word_list.txt").split("\n").map(|s| s.into()).collect()
}

pub fn read_from(file_name: &str) -> Result<Words, (Words, WordListError)> {
    if let Ok(file) = File::open(file_name) {
        let reader = BufReader::new(file);
        let words: Vec<String> = reader.lines()
            .filter_map(io::Result::ok)
            .collect();
        if words.len() == 0 {
            Err((vec![], WordListError::Empty))
        }
        else {
            let words = &words.iter().map(|s| s.as_ref()).collect();
            let words = validate_list(words);
            match words {
                Ok(words)
                    => Ok(words),
                Err((words, invalid))
                    => Err((words, WordListError::InvalidWords { words: invalid }))
            }
        }
    }
    else {
        Err((vec![], WordListError::NoFile))
    }
}

const WORD_RANGE: RangeInclusive<usize> = 3..=7;


pub fn validate_list(words: &Vec<&str>) -> Result<Words, (Words, InvalidWords)> {
    let mut valid: Words = Vec::new();
    let mut invalid: InvalidWords = Vec::new();
    for (pos, word) in words.iter().enumerate() {
        match validate_word(&word) {
            Ok(validated)
                => valid.push(validated),
            Err(errors)
                => invalid.push(InvalidWord {
                    pos,
                    word: word.to_string(),
                    errors
                })
        }
    }

    if invalid.len() == 0 {
        Ok(valid)
    }
    else {
        Err((valid, invalid))
    }
}


pub fn validate_word(word: &str) -> Result<String, WordErrors> {
    let word = word.to_lowercase();

    let mut errors: Vec<WordError> = Vec::new();

    if !WORD_RANGE.contains(&word.len()) {
        errors.push(WordError::InvalidLength{
            len: word.len()
        });
    }
    for (pos, char) in word.chars().enumerate() {
        if !char.is_alphabetic() {
            errors.push(WordError::InvalidCharacter{
                pos, char
            });
        }
    }

    if errors.len() != 0 {
        Err(errors)
    }
    else {
        Ok(word)
    }
}
