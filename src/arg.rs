use clap::{Arg, ArgMatches, Command};
use std::io::stdin;
use std::str::FromStr;
use std::time::Duration;

pub fn parse_args(matches: ArgMatches) -> (Duration, bool) {
    let mut duration = Duration::default();

    // Parse hours to string, and then to u64
    if let Some(time) = matches.get_one::<String>("hours") {
        // * 60 * 60, from hours to seconds
        if let Ok(time) = u64::from_str(time) {
            duration += Duration::from_secs(time * 60 * 60);
        } else {
            eprintln!("--hours has to be a number");
        }
    }

    // Parse minutes to string, and then to u64
    if let Some(time) = matches.get_one::<String>("minutes") {
        // * 60, from minutes to seconds
        if let Ok(time) = u64::from_str(time) {
            duration += Duration::from_secs(time * 60);
        } else {
            eprintln!("--minutes has to be a number");
        }
    }

    // Parse seconds to string, and then to u64
    if let Some(time) = matches.get_one::<String>("seconds") {
        if let Ok(time) = u64::from_str(time) {
            duration += Duration::from_secs(time);
        } else {
            eprintln!("--seconds has to be a number");
        }
    }

    // Parse milliseconds to string, and then to u64
    if let Some(time) = matches.get_one::<String>("milliseconds") {
        if let Ok(time) = u64::from_str(time) {
            duration += Duration::from_millis(time);
        } else {
            eprintln!("--milliseconds has to be a number");
        }
    }

    if duration.as_secs() == 0 {
        println!("Time in seconds:");
        let mut buf = String::new();
        if let Err(_) = stdin().read_line(&mut buf) {
            eprintln!("Unable to read the line");
            std::process::exit(1);
        };
        if let Ok(time) = buf.trim().parse::<u64>() {
            duration += Duration::from_secs(time);
        } else {
            eprintln!("Error: Unable to parse input");
            eprintln!("Input must be a number");
            std::process::exit(1);
        }
    }

    // check if the --quiet flag has been passed
    let quiet = matches.contains_id("quiet");

    (duration, quiet)
}

pub fn get_command() -> Command<'static> {
    Command::new("rime")
        .author("sandalbanditten <sandalbanditten@tutanota.com>")
        .version("0.1")
        .about("A simple timing program for the terminal")
        .arg(
            Arg::new("hours")
                .short('h')
                .long("--hours")
                .takes_value(true)
                .help("The number of hours")
                .default_value("0"),
        )
        .arg(
            Arg::new("minutes")
                .short('m')
                .long("--minutes")
                .takes_value(true)
                .help("The number of minutes")
                .default_value("0"),
        )
        .arg(
            Arg::new("seconds")
                .short('s')
                .long("--seconds")
                .takes_value(true)
                .help("The number of seconds")
                .default_value("0"),
        )
        .arg(
            Arg::new("milliseconds")
                .short('M')
                .long("--milliseconds")
                .takes_value(true)
                .help("The number of milliseconds")
                .default_value("0"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .takes_value(false)
                .help("Wether or not to print a progress bar and progressed time"),
        )
}
