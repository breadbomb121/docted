use std::{env::current_dir, fmt::{self, Display}, fs::{read_to_string, OpenOptions}, io::{Error, Write}, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use anyhow::{anyhow, Result};
pub struct Docted {
    pub project: Project,
    pub notes: NotesFile
}
impl Docted {
    pub fn from_env_dir() -> Result<Self>{
        let mut path = std::env::current_dir()?;
        path.push(".docted");
        Self::from_dir(path)
    }
    pub fn from_dir(mut path: PathBuf) -> Result<Self> {
        
        path.push("docted.toml");
        let project_content = read_to_string(&path)?; 
        let project = toml::from_str(&project_content)?;
        path.pop(); path.push("notes.toml");
        let notes = NotesFile::from_toml_file(path)?;
        
        Ok(Self {project, notes})
    }
}
#[derive(Serialize, Deserialize)]
pub struct Project {
    name: String,
    lang: String
}

#[derive(Deserialize, Serialize)]
pub struct Note {
    timestamp: DateTime<Utc>,
    content: String
}
impl Note {
    pub fn new(content: String) -> Self {
        Note {
            timestamp: Utc::now(),
            content
        }
    }
}
impl Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
#[derive(Serialize, Deserialize)]
pub struct NotesFile {
    pub entries: Vec<Note>
}

impl NotesFile {
    /// Deserialize notes from a TOML file
    pub fn from_toml_file(path: PathBuf) -> Result<Self> {
        let toml_content = read_to_string(path)?;
        toml::from_str(&toml_content).map_err(|e| anyhow!("{}", e))
    }
    pub fn write_env_dir(&self) -> Result<()> {
        let toml_string= toml::to_string(self)?;
        let mut dir = current_dir()?;
        dir.push(".docted/notes.toml");
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&dir)?;
        file.write_all(toml_string.as_bytes())?;
        Ok(())
    }
}

impl Display for NotesFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let note_string = self.entries.iter().enumerate().map(|(i, note)|{
            format!("{}: {}\n", i + 1, note)
        }).fold(String::new(), |acc, s| {
            format!("{}{}", acc, s)
        });
        write!(f, "{}", note_string)
    } 
}
