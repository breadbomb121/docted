use std::io::{Read, stdin};

use crate::cli::LogAction;
use crate::docted::{Docted, Log};
use anyhow::Result;
use bat::{PrettyPrinter, PagingMode};

pub fn exec_logs(action: LogAction) -> Result<()> {
    let mut docted = Docted::from_env_dir()?; 
    match action {
        LogAction::Add  => {
            let mut buf = String::new();
            stdin().read_to_string(&mut buf)?;
            let log= Log::new(buf);
            docted.logs.entries.push(log);
            docted.logs.write_env_dir()?;
        },
        LogAction::View => {
            let logs = docted.logs.to_string();
            PrettyPrinter::new()
                .pager("less")
                .paging_mode(PagingMode::QuitIfOneScreen)
                .header(true)
                .grid(true)
        .input_from_bytes(logs.as_bytes()).print()?;
        },
        LogAction::Export{location} => {
            docted.notes.export(location)?; 
        }
    };
    Ok(())
} 
