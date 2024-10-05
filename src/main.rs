mod notes;
mod cli;
mod docted;
mod web;
mod doc;

use anyhow::Result;
use docted::Docted;
use web::start_server;
use notes::exec_notes;
use doc::get_doc;

use std::{fs::{create_dir, remove_dir_all, File}, io::Write, path::{Path, PathBuf}};

use crate::cli::{Cli, Commands};
use clap::Parser;
fn init_project(name:Option<String>, path: Option<PathBuf>, lang: Option<String>) -> Result<(), std::io::Error> {
    let mut project_path = path.unwrap_or_else(||{ 
        let mut path = PathBuf::new();
            path.push(".");
        path
    }); 
    project_path.push(".docted");
    let file_name = "docted.toml";
    let content = format!(r#"
name = "{}"
lang = "{}"
       "#,
       name.unwrap_or("my_project".into()),
       lang.unwrap_or("".into())
    );

    create_dir(&project_path)?; 
    project_path.push(file_name);
    let mut file = File::create(&project_path)?;
    file.write_all(content.as_bytes())?;
    
    let note_content = "entries = []";
    project_path.pop(); project_path.push("notes.toml");
    let mut notes = File::create(&project_path)?;   
    notes.write_all(&note_content.as_bytes())?;
   Ok(())

}

fn main() -> Result<()>{
    let cli = Cli::parse();

    match cli.command {
        Commands::Init{name, path, lang} => {
            init_project(name, path, lang)?;
        }
        Commands::Remove => {
            remove_dir_all("./.docted")?;
            println!("Removed docted")
        },
        Commands::Doc { item, lang } => get_doc(item, lang)?,
        Commands::Note { action } => exec_notes(action)?,
        Commands::Web => {
            tokio::runtime::Runtime::new()?.block_on(start_server())?;
        }
    };
    Ok(())
}

