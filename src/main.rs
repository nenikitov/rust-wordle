use std::{process::exit, iter::repeat, io};
use colored::{Colorize, Color};

use args::Args;
use words::validate_word;

mod args;
mod wordle;
mod words;
mod tests;


fn main() {
    let validated = words::read_from("res/test.txt");
    println!("{:}", validated.unwrap_err().1);

    /*
    println!(
        "{}",
        match words::validate_list() {
            Ok(words) => todo!(),
            Err((words, filtered)) => todo!(),
        }
    );
    */
    /*
    let args = Args::new();

    if args.help() {
        // Help screen
        println!("{}", Args::HELP_MESSAGE);
        return;
    }
    else {
        // Word screen
        // Word list
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
        // Initialize game
        let game = wordle::WordleGame::new(words);
        let mut score = game.guess_empty();
        let mut guess: String =
            repeat("_")
            .take(score.len())
            .collect();

        //#region Welcome message
        println!(
            "=== Welcome to {}{}{}{}{}{} ===",
            "R".green().on_black(),
            "U".bright_black().on_black(),
            "S".yellow().on_black(),
            "T".bright_black().on_black(),
            "L".green().on_black(),
            "E".yellow().on_black(),
        );
        //#endregion

        //#region Game loop
        loop {
            // Print previous guess
            for (c, s) in guess.chars().zip(score.iter()) {
                let colored =
                    String::from(c)
                    .on_black()
                    .color(match s {
                        wordle::LetterScore::Wrong => Color::BrightBlack,
                        wordle::LetterScore::Present => Color::Yellow,
                        wordle::LetterScore::Correct => Color::Green
                    });
                print!("{}", colored);
            }
            println!();
            // Read input
            let previous_guess = guess.clone();
            guess.clear();
            io::stdin()
                .read_line(&mut guess)
                .expect("Terminal does not support input");
            guess = String::from(guess.trim());
            score = match game.guess(&guess) {
                Ok(s) => s,
                Err(e) => {
                    println!("{}", e.to_string().red());
                    guess = previous_guess;
                    continue;
                }
            };
        }
        //#endregion

        // End screen
    }
    */
}
