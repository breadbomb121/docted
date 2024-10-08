mod app;
mod event;
mod handler;
mod tui;
mod ui;

use std::io;

use crate::docted;

use anyhow::Result;
use app::{App, AppState};
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;

pub async fn start_tui() -> Result<()> {
    let mut app = App::new()?;
    
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;
     
    while app.app_state != AppState::Done {
        tui.draw(&mut app)?;
        let event = tui.events.next().await?;
        match app.app_state{
            AppState::Running => app.normal_mode(event),
            AppState::Done => (),
            AppState::NoteEdit => app.note_edit_mode(event)?
        }
    }
    Ok(())
}
