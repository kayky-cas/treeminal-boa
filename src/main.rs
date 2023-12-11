use std::io::{self, Write};

use crossterm::{
    event::{read, Event},
    terminal, ExecutableCommand,
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let (_w, _h) = terminal::size()?;

    loop {
        match read()? {
            Event::Key(_key) => {}
            _ => {}
        }

        stdout.flush()?;
    }
}
