use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Write},
    iter, vec
};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
    Terminal, text::{Spans, Span},
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
    const SIZE_X: u16 = 5;
    const SIZE_Y: u16 = 3;
    const GAP_X: u16 = 2;
    const GAP_Y: u16 = 1;

    pub fn new(pos: (u16, u16), char: char, score: LetterScore) -> Self {
        Self {
            pos,
            char,
            style: LetterBoxStyle::from(score).to_styles()
        }
    }

    pub fn compute_new_pos(pos: (u16, u16), offset: (u16, u16)) -> (u16, u16) {
        let pos_x =
            pos.0
            + offset.0 * (Self::SIZE_X + Self::GAP_X);
        let pos_y =
            pos.1
            + offset.1 * (Self::SIZE_Y + Self::GAP_Y);
        return (pos_x, pos_y);
    }
    pub fn compute_size(count: (u16, u16)) -> (u16, u16) {
        let size_x =
            if count.0 > 0 {
                (count.0 - 1) * (Self::SIZE_X + Self::GAP_X) + Self::SIZE_X
            }
            else {
                0
            };
        let size_y =
            if count.1 > 0 {
                (count.1 - 1) * (Self::SIZE_Y + Self::GAP_Y) + Self::SIZE_Y
            }
            else {
                0
            };
        (size_x, size_y)
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
                width: Self::SIZE_X,
                height: Self::SIZE_Y
            }
        );
        let paragraph = Paragraph::new(self.char.to_uppercase().to_string())
            .style(self.style.0);
        f.render_widget(
            paragraph,
            Rect {
                x: self.pos.0 + (Self::SIZE_X - 1) / 2,
                y: self.pos.1 + (Self::SIZE_Y - 1) / 2,
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
            LetterBox::new(
                LetterBox::compute_new_pos(self.pos, (i as u16, 0)),
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

        let guess_empty_scores = self.game.guess_empty();
        let word_length = guess_empty_scores.len();

        let minimum_size = LetterBox::compute_size((
            "qwertyuiop".len() as u16,
            (word_length + 1 + 1 + 3) as u16
        ));
        let minimum_size = (minimum_size.0 + 4, minimum_size.1 + 4);

        // Main box
        let main_box = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("RUSTLE")
            .title_alignment(Alignment::Center);
        f.render_widget(main_box, size);

        if size.width >= minimum_size.0 && size.height >= minimum_size.1 {
            // Enough size to draw the game

            // Tries
            // Current guess
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
            let guess_start_x = (size.width - LetterBox::compute_size((word_length as u16, 0)).0) / 2;
            for (i, (word, scores)) in all_guesses.enumerate() {
                LetterBoxWord {
                    pos: LetterBox::compute_new_pos((guess_start_x, 2), (0, i as u16)),
                    word,
                    scores
                }.render(f);
            }

            // Keyboard
            let keyboard_size = LetterBox::compute_size(("qwertyuiop".len() as u16, 3));
            for (i, row) in ["qwertyuiop", "asdfghjkl", "zxcvbnm"].iter().enumerate() {
                let scores = self.game.known_guesses(row);
                let pos_x =
                    (size.width - keyboard_size.0) / 2
                    + i as u16 * (LetterBox::SIZE_X - 1);
                let pos_y =
                    size.height - 2
                    - LetterBox::compute_size((0, (3 - i) as u16)).1;

                LetterBoxWord {
                    pos: (pos_x, pos_y),
                    word: row,
                    scores: &scores
                }.render(f);
            }

            // Error message
            let error = Paragraph::new(self.error.clone())
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center);
            f.render_widget(error, Rect {
                x: 2,
                y: size.height - 2 - keyboard_size.1 - 2,
                width: size.width - 2,
                height: 2
            });
        }
        else {
            // Error message box
            let color_x = if size.width < minimum_size.0 { Color::LightRed } else { Color::LightGreen };
            let color_y = if size.height < minimum_size.1 { Color::LightRed } else { Color::LightGreen };

            let text = vec![
                Spans::from(Span::raw("Terminal window is too small")),
                Spans::from(vec![
                    Span::raw("Width = "),
                    Span::styled(format!("{}", size.width), Style::default().fg(color_x)),
                    Span::raw(format!(" (needed {})", minimum_size.0)),
                ]),
                Spans::from(vec![
                    Span::raw("Height = "),
                    Span::styled(format!("{}", size.height), Style::default().fg(color_y)),
                    Span::raw(format!(" (needed {})", minimum_size.1)),
                ]),
            ];

            let paragraph = Paragraph::new(text)
                .style(Style::default().bg(Color::Black))
                .alignment(Alignment::Center);
            f.render_widget(paragraph, size);
        }
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
        if self.guess.len() < self.game.guess_empty().len() {
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
