use anyhow::{anyhow, Result};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::ListState;

use crate::docted::Docted;

use crate::tui::event::Event;

#[derive(PartialEq, Eq)]
pub enum AppState{
    Running,
    NoteEdit,
    Done
}

pub struct App {
    pub app_state: AppState,
    docted: Docted,
    pub note_index: ListState,
    pub buffer: String 
}

impl App {
    pub fn new() -> Result<App> {
        Ok(Self { 
            app_state: AppState::Running, 
            note_index: ListState::default(), 
            docted: Docted::from_env_dir()?, 
            buffer: String::new() 
        })
    }

    pub fn tick(&self){}

    pub fn quit(&mut self) {
        self.app_state = AppState::Done
    }
    pub fn update_buffer(&mut self){
        if let Some(index) = self.note_index.selected(){
            if let Some(note) = self.docted.notes.entries.get(index){
                self.buffer = note.to_string()
            }else {
                self.buffer = String::new()
            }
        }
    }
    pub fn note_list_up(&mut self){
        self.note_index.select_previous();
        self.update_buffer();
    }

    pub fn note_list_down(&mut self){
       self.note_index.select_next();
       self.update_buffer();
    }

    pub fn edit_note(&mut self, key_event:KeyEvent) -> Result<()>{
        match key_event.code {
            KeyCode::Char(c) => {
                self.buffer.push(c);
            },
            KeyCode::Esc => {
                self.app_state = AppState::Running
            },
            KeyCode::Backspace => {self.buffer.pop();},
            KeyCode::Enter => {
                let index = self.note_index.selected().ok_or(anyhow!("Unable to get index"))?;
                if index == self.docted.notes.entries.len(){
                    self.docted.add_note(self.buffer.clone())?;
                }else {
                    self.docted.notes.edit(index, self.buffer.clone())?;
                }
                self.app_state = AppState::Running
            }
            _ => ()
        } 

        Ok(())
    }
    pub fn note_edit_mode(&mut self, event: Event ) -> Result<()> {
        if self.app_state != AppState::NoteEdit {
            return Err(anyhow!("Function is linked to a mode"))
        }
        match event {
            Event::Tick => (),
            Event::Key(key_event)=> self.edit_note(key_event)?,
            Event::Resize(_, _) => {},
            Event::Mouse(_) => (),
        }
        Ok(())
    }
    pub fn normal_mode(&mut self, event: Event) {
        match event{
            Event::Tick => (),
            Event::Resize(_, _) => (),
            Event::Mouse(_) => (),
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Esc => {self.quit()}
                    KeyCode::Char('k') => {self.note_list_up()},
                    KeyCode::Char('j') => {self.note_list_down()},
                    KeyCode::Char('i') => self.app_state = AppState::NoteEdit,
                    _ => ()
                }
            }
        }
    }
    pub fn note_entries(&mut self) -> Vec<String> {
        let mut items: Vec<String> = self.docted.notes.entries.iter().map(|x| x.to_string()).collect();
        items.push("New Note".to_string());
        items
    }
}

