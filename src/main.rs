mod task;
mod shell;

use std::io::{self, stdin, stdout, BufReader, prelude::*};
use task::Task;

const PROMPT: &str = ": ";

fn prompt() -> io::Result<()> {
    let mut stdout = stdout();

    stdout.write_all(PROMPT.as_bytes())?;
    stdout.flush()?;

    Ok(())
}

fn main() -> io::Result<()> {
    let mut shell = shell::Shell::new();
    let stdin = stdin();

    prompt()?;
    for line in BufReader::new(stdin.lock()).lines() {
        shell.run(line?.into());
        prompt()?;
    }

    Ok(())
}
