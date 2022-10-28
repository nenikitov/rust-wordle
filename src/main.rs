use std::path::Path;

mod words;
mod wordle;


const PATH_ALL_WORDS_FILE: &str = "res/words_alpha.txt";
const PATH_WORDS_FILE: &str = "res/words_filtered.txt";


fn main() {
    if !Path::new(PATH_WORDS_FILE).exists() {
        println!("Game can't find the words file, generating new");
        let words = words::read_from(PATH_ALL_WORDS_FILE);
        let words = words::filter(words);
        words::write_into(PATH_WORDS_FILE, &words);
    }
    let words = words::read_from("res/words_filtered.txt");
    let game = wordle::WordleGame::new(words);
}
