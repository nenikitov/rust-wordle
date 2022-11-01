use std::{
    fs::File,
    io::{
        BufReader,
        BufRead,
        self
    }, fmt::Display
};

pub enum WordIoError {
    NoFile,
    InvalidFormat
}

impl Display for WordIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFile => write!(f, "Specified file cannot be read/does not exist"),
            Self::InvalidFormat => write!(f, "Word list format is improperly formatted"),
        }
    }
}


pub fn default_words() -> Vec<String> {
    include_str!("../res/word_list.txt").split("\n").map(|s| s.into()).collect()
}

pub fn read_from(file_name: &str) -> Result<Vec<String>, WordIoError> {
    if let Ok(file) = File::open(file_name) {
        let reader = BufReader::new(file);
        let words: Vec<String> = reader.lines()
            .filter_map(io::Result::ok)
            .collect();
        if words.len() == 0 {
            Err(WordIoError::InvalidFormat)
        }
        else {
            Ok(words)
        }
    }
    else {
        Err(WordIoError::NoFile)
    }
}

pub fn count_all(word: &String, set: &str) -> usize {
    word.chars().filter(|char| {
        set.contains(*char)
    }).count()
}
