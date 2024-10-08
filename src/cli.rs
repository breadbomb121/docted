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
    ///Creates a .docted folder
    Init{
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long, value_name = "DIR")]
        path: Option<PathBuf>,
        
        #[arg(short, long, value_name = "LANG")]
        lang: Option<String>
    },
    Tui,
    ///Finds standard documentation
    Doc {
        /// The format for the documentation (e.g., "html", "markdown")
        item: String,
        #[arg(short, long, value_name = "LANG")]
        lang: Option<String>,

        #[clap(long, short, action)]
        no_page: bool
    },
    ///Removes .docted folder
    Remove,
    ///Utility for note taking
    Note{
        #[command(subcommand)]
        action: NoteAction
    },
    ///Utility for general logging
    Log{
        #[command(subcommand)]
        action: LogAction 
    }
    
    
}
#[derive(Clone, Subcommand)]
pub enum NoteAction{
    //Adds a new note
    Add{
        content: String
    },
    //Removes a note
    Remove{
        ///The id to remove
        id: usize
    },
    ///Views all notes
    View {
        #[clap(long, short, action)]
        no_page: bool
    },
    //Exports notes as markdown list
    Export {
        location: PathBuf
    }
}

#[derive(Clone, Subcommand)]
pub enum LogAction{
    ///Adds a new log from stdin
    Add,
    ///Shows all logs
    View{
        #[clap(long, short, action)]
        no_page: bool
    },
    ///Exports logs to path
    Export {
        location: PathBuf
    }
}




