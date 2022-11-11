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
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
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
    Close
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

        LetterBox::new((3, 3), 'w', LetterScore::Unknown).render(f);
        LetterBox::new((9, 3), 'o', LetterScore::Wrong).render(f);
        LetterBox::new((15, 3), 'r', LetterScore::Present).render(f);
        LetterBox::new((21, 3), 'd', LetterScore::Correct).render(f);
        LetterBox::new((27, 3), 'l', LetterScore::Present).render(f);
        LetterBox::new((33, 3), 'e', LetterScore::Unknown).render(f);
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
                                self.state = AppState::End(AppEndState::Close)
                            }
                            else {
                                self.add_to_input(char)
                            }
                        },
                        KeyCode::Backspace =>
                            self.remove_from_input(),
                        KeyCode::Esc =>
                            self.state = AppState::End(AppEndState::Close),
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
        self.guess.push(char);
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
