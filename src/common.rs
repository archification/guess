extern crate crossterm;

use std::io::stdout;
use crossterm::{
    ExecutableCommand,
    cursor,
    Result,
    terminal::{
        Clear,
        ClearType
    }
};

pub fn clear() -> Result<()> {
    stdout()
        .execute(Clear(ClearType::All))?
        .execute(cursor::MoveTo(0, 0))?;
    Ok(())
}
