use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    terminal, QueueableCommand,
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::Hide)?
        .flush()?;

    let (_w, _h) = terminal::size()?;

    loop {
        if poll(Duration::from_millis(20))? {
            match read()? {
                Event::Key(key) => {
                    println!("{:?}", key);

                    match key.code {
                        KeyCode::Char('q') => break,
                        _ => {}
                    }

                    stdout.flush()?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
