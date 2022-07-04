use std::io;
use std::time::Duration;
use termion::{clear, cursor};

pub fn print_progress(
    screen: &mut impl io::Write,
    i: u16,
    duration: Duration,
    elapsed: Duration,
) -> io::Result<()> {
    // Go to start of first line
    write!(screen, "{}", cursor::Goto(1, 1)).unwrap();
    // Clear first line
    write!(screen, "{}", clear::CurrentLine).unwrap();
    // Print elapsed time
    write!(screen, "Time elapsed: {:.0?}", duration - elapsed).unwrap();
    // Go to position of printing time left and print time left
    write!(screen, "{}", cursor::Goto(1, 2)).unwrap();
    write!(screen, "Time remaining: {:.0?}", elapsed).unwrap();

    // Only print to bar, if a percent of progress has been made
    if i % 10 == 0 {
        write!(screen, "{}", cursor::Goto(1 + i / 10, 3)).unwrap();
        write!(screen, "#").unwrap();
    }

    screen.flush().unwrap();

    Ok(())
}

pub fn initial_print(screen: &mut impl io::Write) -> io::Result<()> {
    write!(screen, "{}", cursor::Hide)?;
    write!(screen, "{}", cursor::Goto(1, 1))?;
    write!(screen, "Time elapsed:")?;
    write!(screen, "{}", cursor::Goto(1, 2))?;
    write!(screen, "Time remaining:")?;
    write!(screen, "{}[{}]", cursor::Goto(1, 3), cursor::Goto(101, 3),)?;
    write!(screen, "{}", cursor::Goto(2, 3))?;

    screen.flush()?;

    Ok(())
}
