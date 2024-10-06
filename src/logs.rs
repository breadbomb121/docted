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
        LogAction::View{no_page} => {
            let logs = docted.logs.to_string();
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
            printer.input_from_bytes(logs.as_bytes()).print()?;

        },
        LogAction::Export{location} => {
            docted.notes.export(location)?; 
        }
    };
    Ok(())
} 
