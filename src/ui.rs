use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Write},
    iter
};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
    Terminal,
};
use crate::wordle::{self, LetterScore};


#[derive(
    Debug,
    Clone, Copy,
    PartialEq, PartialOrd, Eq
)]
pub enum AppEndState {
    Won,
    Lost,
    Close{forced: bool}
}

#[derive(
    Debug,
    Clone, Copy,
    PartialEq, PartialOrd, Eq
)]
pub enum AppState {
    InProgress,
    End(AppEndState)
}



pub trait Drawable {
    fn render<B: Backend>(&self, f: &mut Frame<B>);
}



struct LetterBoxStyle {
    background: Color,
    borders: Borders
}
impl From<LetterScore> for LetterBoxStyle {
    fn from(score: LetterScore) -> Self {
        match score {
            LetterScore::Unknown =>
                Self {
                    background: Color::DarkGray,
                    borders: Borders::BOTTOM
                },
            LetterScore::Wrong =>
                Self {
                    background: Color::Black,
                    borders: Borders::NONE
                },
            LetterScore::Present =>
                Self {
                    background: Color::Yellow,
                    borders: Borders::LEFT | Borders::RIGHT
                },
            LetterScore::Correct =>
                Self {
                    background: Color::Green,
                    borders: Borders::ALL
                }
        }
    }
}
impl LetterBoxStyle {
    pub fn to_styles(&self) -> (Style, Borders) {
        (
            Style {
                fg: Some(Color::White),
                bg: Some(self.background),
                add_modifier: Modifier::BOLD,
                sub_modifier: Modifier::empty()
            },
            self.borders
        )
    }
}



struct LetterBox {
    pos: (u16, u16),
    char: char,
    style: (Style, Borders)
}
impl LetterBox {
    pub fn new(pos: (u16, u16), char: char, score: LetterScore) -> Self {
        Self {
            pos,
            char,
            style: LetterBoxStyle::from(score).to_styles()
        }
    }
}
impl Drawable for LetterBox {
    fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let block = Block::default()
            .style(self.style.0)
            .borders(self.style.1)
            .border_type(BorderType::Thick);
        f.render_widget(
            block,
            Rect {
                x: self.pos.0,
                y: self.pos.1,
                width: 5,
                height: 3
            }
        );
        let paragraph = Paragraph::new(self.char.to_uppercase().to_string())
            .style(self.style.0);
        f.render_widget(
            paragraph,
            Rect {
                x: self.pos.0 + 2,
                y: self.pos.1 + 1,
                width: 1,
                height: 1
            }
        )
    }
}


struct LetterBoxWord<'a> {
    pos: (u16, u16),
    word: &'a str,
    scores: &'a [wordle::LetterScore]
}
impl Drawable for LetterBoxWord<'_> {
    fn render<B: Backend>(&self, f: &mut Frame<B>) {
        for i in 0..self.word.len() {
            let pos_x: u16 = self.pos.0 + (i as u16) * 7;
            LetterBox::new(
                (pos_x, self.pos.1),
                self.word.chars().nth(i).unwrap(),
                *self.scores.iter().nth(i).unwrap()
            ).render(f);
        }
    }
}




pub struct App {
    game: wordle::WordleGame,
    guess: String,
    error: String,
    tries: Vec<(String, Vec<wordle::LetterScore>)>,
    state: AppState,
}

impl Drawable for App {
    fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let size = f.size();

        // Main box
        let main_box = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("RUSTLE")
            .title_alignment(Alignment::Center);
        f.render_widget(main_box, size);

        // Tries
        // Current guess
        let guess_empty_scores = self.game.guess_empty();
        let word_length = guess_empty_scores.len();
        let guess_padded: String =
            (0..word_length)
                .map(|i| self.guess.chars().nth(i).unwrap_or(' ')).collect();
        let guess_scores: Vec<wordle::LetterScore> =
            guess_padded.chars()
                .map(
                    |c|
                    if c.is_alphabetic() {
                        LetterScore::Unknown
                    }
                    else {
                        LetterScore::Wrong
                    }
                ).collect();
        // Future guesses
        let guess_empty = str::repeat(" ", word_length);
        // All guesses
        let current = (guess_padded, guess_scores);
        let future = (guess_empty, guess_empty_scores);
        let all_guesses =
            self.tries.iter()
                .chain(iter::repeat(&current).take(1))
                .chain(iter::repeat(&future).take(self.game.lives()));

        for (i, (word, scores)) in all_guesses.enumerate() {
            let pos_y = 4 + (i as u16) * 4;
            LetterBoxWord {
                pos: (4, pos_y),
                word,
                scores
            }.render(f);
        }

        // Keyboard
        for (i, row) in ["qwertyuiop", "asdfghjkl", "zxcvbnm"].iter().enumerate() {
            let scores  = self.game.known_guesses(row);
            let pos_x = 4 + (word_length as u16) * 7 + 3 * (i as u16);
            let pos_y = 4 + (i as u16) * 4;

            LetterBoxWord {
                pos: (pos_x, pos_y),
                word: row,
                scores: &scores
            }.render(f);
        }
        // Current guess
    }
}

impl App {
    pub fn new(game: wordle::WordleGame) -> Self {
        Self {
            game,
            guess: "".to_string(),
            error: "".to_string(),
            tries: Vec::new(),
            state: AppState::InProgress,
        }
    }

    pub fn update(&mut self) {
        if let Ok(event) = event::read() {
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Enter =>
                            self.submit_input(),
                        KeyCode::Char(char) => {
                            if key.modifiers == KeyModifiers::CONTROL && char == 'c' {
                                self.state = AppState::End(AppEndState::Close { forced: true })
                            }
                            else {
                                self.add_to_input(char)
                            }
                        },
                        KeyCode::Backspace =>
                            self.remove_from_input(),
                        KeyCode::Esc =>
                            self.state = AppState::End(AppEndState::Close { forced: false }),
                        _ => ()
                    }
                }
            }
        }
    }

    pub fn state(&self) -> AppState {
        self.state
    }

    fn add_to_input(&mut self, char: char) {
        if (self.guess.len() < self.game.guess_empty().len()) {
            self.guess.push(char);
        }
    }

    fn remove_from_input(&mut self) {
        self.guess.pop();
    }

    fn submit_input(&mut self) {
        if self.state == AppState::InProgress {
            let score = self.game.guess(self.guess.as_str());
            match score {
                Ok(score) => {
                    /*
                    if let Some(_) = score.iter().filter(|&s| *s != wordle::LetterScore::Correct).next() {
                        self.state = AppState::End(AppEndState::Won);
                    }
                    else if self.game.lives() == 0 {
                        self.state = AppState::End(AppEndState::Lost);
                    }
                    */
                    self.tries.push((self.guess.clone(), score));
                    self.guess.clear();
                    self.error.clear();
                }
                Err(error) => {
                    self.guess.clear();
                    self.error = error.to_string();
                }
            }
        }
    }
}


pub fn start_ui<B: Backend>(backend: B) -> Result<Terminal<B>, io::Error>
    where B: Backend
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen
    )?;
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

pub fn end_ui<B: Backend + Write>(mut terminal: Terminal<B>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    let backend = terminal.backend_mut();
    execute!(
        backend,
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    Ok(())
}
