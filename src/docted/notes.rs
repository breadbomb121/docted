use crate::cli::NoteAction;
use crate::docted::{Docted, Note};
use anyhow::Result;
use bat::{PrettyPrinter, PagingMode};
pub fn exec_notes(action: NoteAction) -> Result<()> {
    let mut docted = Docted::from_env_dir()?; 
    match action {
        NoteAction::Add { content } => {
            let note = Note::new(content);
            docted.notes.entries.push(note);
            docted.notes.write_env_dir()?;
        },
        NoteAction::Remove { id } => {
            let mut notes = docted.notes;
            if id != 0 && id - 1 < notes.entries.len() {
                notes.entries.remove(id - 1);
                notes.write_env_dir()?;
            }
        },
        NoteAction::View{no_page} => {
            let notes = docted.notes.to_string();
            let mut printer = PrettyPrinter::new();
                printer
                    .pager("less")
                    .header(true)
                    .grid(true);
            if no_page {
                printer.paging_mode(PagingMode::Never);
            }else {
                printer.paging_mode(PagingMode::QuitIfOneScreen);
            }
            printer.input_from_bytes(notes.as_bytes()).print()?;
        },
        NoteAction::Export{location} => {
            docted.notes.export(location)?; 
        }
    };
    Ok(())
} 


