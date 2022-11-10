use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Write},
    collections::HashMap
};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
    Terminal,
};
use crate::wordle;


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


pub struct App {
    game: wordle::WordleGame,
    guess: String,
    error: String,
    tries: Vec<(String, Vec<wordle::LetterScore>)>,
    state: AppState,
    styles: HashMap<wordle::LetterScore, Style>
}

impl App {
    pub fn new(game: wordle::WordleGame) -> Self {
        Self {
            game,
            guess: "".to_string(),
            error: "".to_string(),
            tries: Vec::new(),
            state: AppState::InProgress,
            styles: [
                (
                    wordle::LetterScore::Wrong,
                    Style {
                        fg: Some(Color::DarkGray),
                        bg: Some(Color::Black),
                        add_modifier: Modifier::BOLD | Modifier::UNDERLINED,
                        sub_modifier: Modifier::empty()
                    }
                ),
                (
                    wordle::LetterScore::Present,
                    Style {
                        fg: Some(Color::LightYellow),
                        bg: Some(Color::Black),
                        add_modifier: Modifier::BOLD | Modifier::UNDERLINED,
                        sub_modifier: Modifier::empty()
                    }
                ),
                (
                    wordle::LetterScore::Correct,
                    Style {
                        fg: Some(Color::LightGreen),
                        bg: Some(Color::Black),
                        add_modifier: Modifier::BOLD | Modifier::UNDERLINED,
                        sub_modifier: Modifier::empty()
                    }
                )
            ].iter().cloned().collect()
        }
    }

    fn get_style(&self, score: wordle::LetterScore) -> Style {
        *self.styles.get(&score).unwrap()
    }

    pub fn submit_input(&mut self) {
        if self.state == AppState::InProgress {
            let score = self.game.guess(self.guess.as_str());
            match score {
                Ok(score) => {
                    if let Some(_) = score.iter().filter(|&s| *s != wordle::LetterScore::Correct).next() {
                        self.state = AppState::End(AppEndState::Won);
                    }
                    else if self.game.lives() == 0 {
                        self.state = AppState::End(AppEndState::Lost);
                    }
                    self.tries.push((self.guess.clone(), score));
                    self.error.clear();
                }
                Err(error) => {
                    self.guess.clear();
                    self.error = error.to_string();
                }
            }
        }
    }


    pub fn add_to_input(&mut self, char: char) {
        self.guess.push(char);
    }

    pub fn remove_from_input(&mut self) {
        self.guess.pop();
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

    pub fn render<B: Backend>(&self, f: &mut Frame<B>) {
        let size = f.size();

        let title = Spans::from(vec![
            Span::styled("R", self.get_style(wordle::LetterScore::Correct)),
            Span::styled("U", self.get_style(wordle::LetterScore::Wrong)),
            Span::styled("S", self.get_style(wordle::LetterScore::Present)),
            Span::styled("T", self.get_style(wordle::LetterScore::Wrong)),
            Span::styled("L", self.get_style(wordle::LetterScore::Correct)),
            Span::styled("E", self.get_style(wordle::LetterScore::Wrong))
        ]);

        let main_box = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(title)
            .title_alignment(Alignment::Center);

        f.render_widget(main_box, size);

        let items = [
            ListItem::new("Item 1"),
            ListItem::new("Item 2"),
            ListItem::new("Item 3")
        ];
        let tries = List::new(items)
            .block(Block::default().title("List").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));

        f.render_widget(tries, Rect {
            x: 1,
            y: 1,
            width: size.width - 2,
            height: (size.height - 2)
        });
    }

    pub fn state(&self) -> AppState {
        self.state
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
