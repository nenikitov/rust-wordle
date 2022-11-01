use std::{path::Path, process::exit};

use args::Args;

mod args;
mod wordle;
mod words;


fn main() {
    // Parse arguments
    let args = Args::new();

    if args.help() {
        println!("{}", Args::HELP_MESSAGE);
        return;
    }
    let words =
        if let Some(words) = args.word_list() {
            match words::read_from(words) {
                Ok(words) => words,
                Err(error) => {
                            println!("{error}");
                            exit(1);
                        }
            }
        } else { 
            words::default_words()
        };
    println!("{}", words.len());
    let game = wordle::WordleGame::new(words);
}
