use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "docted", version = "1.0", author = "Peter Berndtsson")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init{
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long, value_name = "DIR")]
        path: Option<PathBuf>,
        
        #[arg(short, long, value_name = "LANG")]
        lang: Option<String>
    },
    Doc {
        /// The format for the documentation (e.g., "html", "markdown")
        item: String,
        #[arg(short, long, value_name = "LANG")]
        lang: Option<String>
    },
    Remove,
    Note{
        #[command(subcommand)]
        action: NoteAction
    },
    Web,
    Log{
        #[command(subcommand)]
        action: LogAction 
    }
    
    
}
#[derive(Clone, Subcommand)]
pub enum NoteAction{
    Add{
        content: String
    },
    Remove{
        id: usize
    },
    View,
    Export {
        location: PathBuf
    }
}

#[derive(Clone, Subcommand)]
pub enum LogAction{
    Add,
    View,
    Export {
        location: PathBuf
    }
}




