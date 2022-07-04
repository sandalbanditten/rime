use crate::arg::{get_command, parse_args};
use crate::ui::{initial_print, print_progress};

use std::io::{stdout, Write};
use std::thread;
use termion::cursor;
use termion::screen::AlternateScreen;
use termion::terminal_size;

mod arg;
mod ui;

// TODO: Terminal bells
// Visualisation?
fn main() {
    let command = get_command();
    let matches = command.get_matches();
    let (duration, quiet) = parse_args(matches);

    if quiet {
        thread::sleep(duration);
    } else {
        let step = duration / 1000;

        let (width, _) = terminal_size().expect("Unable to get terminal size");
        // TODO: implement smol mode for smol terminals
        if width < 102 {
            eprintln!("Terminal is not wide enough!");
            std::process::exit(1);
        }

        let mut screen = AlternateScreen::from(stdout());

        if let Err(err) = initial_print(&mut screen) {
            eprintln!("Error when printing to the screen: {}", err);
        }

        let mut elapsed = duration;

        for i in 1..=1000 {
            if let Err(err) = print_progress(&mut screen, i, duration, elapsed) {
                eprintln!("Error when printing to the screen: {}", err);
            }
            elapsed -= step;
            std::thread::sleep(step);
        }

        write!(screen, "{}", cursor::Show).unwrap();
        screen.flush().unwrap();
    }
}
