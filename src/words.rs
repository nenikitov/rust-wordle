use std::{
    fs::File,
    io::{
        BufReader,
        BufRead,
        self,
        BufWriter,
        Write
    }
};


const VOWELS: &str = "aeiouy";
const CONSONANTS: &str = "bcdfghjklmnpqrstvwxz";


pub fn write_into(file_name: &str, words: &Vec<String>) {
    let file = File::create(file_name).expect("File not found");
    let mut writer = BufWriter::new(file);
    writer.write_all(words.join("\n").as_bytes()).expect("File could not be written");
}

pub fn read_from(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);
    reader.lines()
        .filter_map(io::Result::ok)
        .collect()
}

pub fn filter(words: Vec<String>) -> Vec<String> {
    words
        .into_iter()
        .filter(is_a_valid_word)
        .collect()
}

fn is_a_valid_word(word: &String) -> bool {
    let len = word.len();
    if (4..8).contains(&len) {
        let consecutive_vowels = count_consecutive(word, VOWELS);
        let consecutive_consonants = count_consecutive(word, CONSONANTS);
        let percent_vowels = count_all(word, VOWELS) as f64 / len as f64;
        let percent_consonants = count_all(word, CONSONANTS) as f64 / len as f64;

        consecutive_consonants <= 4
            && consecutive_vowels <= 2
            && percent_vowels <= 0.9
            && percent_consonants <= 0.9
    }
    else {
        false
    }
}

pub fn count_all(word: &String, set: &str) -> usize {
    word.chars().filter(|char| {
        set.contains(*char)
    }).count()
}

fn count_consecutive(word: &String, set: &str) -> usize {
    let mut max: usize = 0;
    let mut current: usize = 0;

    for char in word.chars() {
        if set.contains(char) {
            current += 1;
        }
        else if current > max {
            max = current;
            current = 0;
        }
    }
    max
}
