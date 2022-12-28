use crate::{db, Card};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use time::OffsetDateTime;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};
use tui::{Frame, Terminal};

enum UiState {
    Hidden,
    Shown,
}

pub fn run<B>(terminal: &mut Terminal<B>, mut db: db::Database, now: OffsetDateTime) -> Result<()>
where
    B: Backend,
{
    let mut state = UiState::Hidden;
    let Some(mut current) = db.candidate(&now) else { return Ok(()); };

    loop {
        terminal.draw(|f| draw(f, current.card, &state))?;

        if let Event::Key(key) = event::read()? {
            match state {
                UiState::Hidden => match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Char('s') => {
                        state = UiState::Shown;
                    }
                    _ => {}
                },
                UiState::Shown => {
                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('0') => {
                            current.state.update(0);
                            state = UiState::Hidden;
                        }
                        KeyCode::Char('1') => {
                            current.state.update(1);
                            state = UiState::Hidden;
                        }
                        KeyCode::Char('2') => {
                            current.state.update(2);
                            state = UiState::Hidden;
                        }
                        KeyCode::Char('3') => {
                            current.state.update(3);
                            state = UiState::Hidden;
                        }
                        KeyCode::Char('4') => {
                            current.state.update(4);
                            state = UiState::Hidden;
                        }
                        KeyCode::Char('5') => {
                            current.state.update(5);
                            state = UiState::Hidden;
                        }
                        _ => {}
                    };

                    db::write(&db)?;

                    if let Some(new) = db.candidate(&now) {
                        current = new;
                    } else {
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn draw<B>(f: &mut Frame<B>, card: &Card, state: &UiState)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Max(200),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new(&*card.title)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(title, chunks[0]);

    let separator = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::DarkGray));

    f.render_widget(separator, chunks[1]);

    if matches!(state, UiState::Shown) {
        let text = Paragraph::new(&*card.body);
        f.render_widget(text, chunks[2]);
    }

    let command_line = match state {
        UiState::Hidden => Paragraph::new("[q] quit  [s] show card"),
        UiState::Shown => Paragraph::new(
            "[q] quit  [0] again  [1] hard  [2] foo  [3] effort  [4] hesitate  [5] easy",
        ),
    };

    f.render_widget(command_line, chunks[3]);
}
