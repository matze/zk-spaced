use anyhow::{Context, Result};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use serde::Deserialize;
use std::io;
use time::OffsetDateTime;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod db;
mod ui;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Card {
    filename: String,
    title: String,
    body: String,
}

fn main() -> Result<()> {
    let cards: Vec<Card> = serde_json::from_reader(io::stdin())
        .with_context(|| "Could not read, perhaps no card is tagged?")?;

    let dirs = xdg::BaseDirectories::with_prefix("zk-spaced")?;
    let db_path = dirs.place_data_file("db.json")?;
    let now = OffsetDateTime::now_utc();
    let db = db::Database::open(db_path, now, cards)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    ui::run(&mut terminal, db, now)?;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
