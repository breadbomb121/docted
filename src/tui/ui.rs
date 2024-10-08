use ratatui::{
    layout::{Constraint, Direction, Layout}, prelude::*, style::{Color, Style}, widgets::{self, Block, Paragraph, Wrap}, Frame
};

use crate::tui::app::App;

use super::app::AppState;

pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::new(Direction::Horizontal, vec![
        Constraint::Percentage(20),
        Constraint::Percentage(80)
    ]) 
    .split(frame.area());
    let list = widgets::List::new(app.note_entries()) 
        .block(Block::bordered().title("List"))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);
    let paragraph_block_style = if app.app_state == AppState::NoteEdit{
        Style::new().fg(Color::Red)
    }else {
        Style::new().fg(Color::Blue)
    };
    let paragraph = Paragraph::new(app.buffer.clone())
        .block(Block::bordered().title("Note"))
        .style(paragraph_block_style)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });


    //Side List
    frame.render_stateful_widget( 
        list, 
        layout[0],
        &mut app.note_index
    );
    frame.render_widget(paragraph, layout[1]);
}
