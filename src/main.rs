mod args;
mod tests;
mod ui;
mod wordle;
mod words;



use std::{io::{self, Read}};

use colored::Colorize;
use crossterm::style::Stylize;
use tui::{backend::CrosstermBackend};

use args::Args;



fn main() -> Result<(), i32> {
    let args = Args::new();

    if args.help() {
        // Help screen
        println!("{}", Args::HELP_MESSAGE);
        return Ok(())
    }
    else {
        // Get words
        let words =
            if let Some(path) = args.word_list() {
                match words::read_from(path) {
                    Ok(words) =>
                        words,
                    Err((words, errors)) => {
                        eprintln!("{}", errors.to_string().yellow());
                        if words.len() > 0 {
                            println!("{}", Colorize::yellow("There are still words left in the word list, playing"));
                            println!("{}", Colorize::yellow("Press ENTER to continue"));
                            io::stdin().read(&mut [0]).unwrap();
                            words
                        }
                        else {
                            eprintln!("{}", Colorize::red("No word list to play with"));
                            return Err(1);
                        }
                    }
                }
            }
            else {
                words::default_words()
            };

        let mut app = ui::App::new(
            wordle::WordleGame::new(words)
        );

        let mut terminal = if let Ok(terminal) = ui::start_ui(CrosstermBackend::new(io::stdout())) {
            terminal
        } else {
            eprintln!("{}", Colorize::red("Can't initialize TUI session"));
            return Err(1);
        };

        while app.state() == ui::AppState::InProgress {
            terminal.draw(|f| app.render(f)).unwrap();
            app.update();
        }

        ui::end_ui(terminal).unwrap();

        Ok(())
    }
}
