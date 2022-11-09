mod args;
mod wordle;
mod words;
mod tests;



use std::{
    process::exit,
    iter,
    io, error
};

use colored::{
    Colorize,
    Color
};

use args::Args;




fn main() -> Result<(), i32>{
    let args = Args::new();

    if args.help() {
        // Help screen
        println!("{}", Args::HELP_MESSAGE);
        return Ok(())
    }
    else {
        // Word screen
        // Word list
        let words =
            if let Some(path) = args.word_list() {
                match words::read_from(path) {
                    Ok(words) =>
                        words,
                    Err((words, errors)) => {
                        eprintln!("{}", errors.to_string().yellow());
                        if words.len() > 0 {
                            print!("{}", "There are still words left in the word list, playing");
                            words
                        }
                        else {
                            eprintln!("{}", "No word list to play with".red());
                            return Err(1);
                        }
                    }
                }
            }
            else {
                words::default_words()
            };

        // Initialize game
        let mut game = wordle::WordleGame::new(words);
        let mut score = game.guess_empty();
        let mut guess: String =
            iter::repeat("_")
            .take(score.len())
            .collect();

        //#region Welcome message
        println!(
            "=== Welcome to {}{}{}{}{}{} ===",
            "R".on_black().green(),
            "U".on_black().bright_black(),
            "S".on_black().yellow(),
            "T".on_black().bright_black(),
            "L".on_black().green(),
            "E".on_black().yellow()
        );
        //#endregion

        //#region Game loop
        loop {
            // Print previous guess
            for (c, s) in guess.chars().zip(score.iter()) {
                print_letter(c, &s);
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

    fn print_letter(c: char, score: &wordle::LetterScore) {
        let colored =
            String::from(c)
            .on_black()
            .color(match score {
                wordle::LetterScore::Wrong => Color::BrightBlack,
                wordle::LetterScore::Present => Color::Yellow,
                wordle::LetterScore::Correct => Color::Green
            });
        print!("{}", colored);
    }
}
