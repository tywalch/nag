use clap::{App, Arg};
use std::process::Command;
use std::thread;
use std::time::{Duration};
use chrono::{DateTime};
use chrono::prelude::Local;

fn parse_duration(s: &str) -> Result<Duration, &'static str> {
    let parts: Vec<&str> = s.split(':').collect();
    let duration = match parts.len() {
        1 => {
            let minutes = parts[0].parse::<u64>().map_err(|_| "Invalid minutes")?;
            Duration::from_secs(minutes * 60)
        },
        2 => {
            let minutes = parts[0].parse::<u64>().map_err(|_| "Invalid minutes")?;
            let seconds = parts[1].parse::<u64>().map_err(|_| "Invalid seconds")?;
            Duration::from_secs(minutes * 60 + seconds)
        },
        3 => {
            let hours = parts[0].parse::<u64>().map_err(|_| "Invalid hours")?;
            let minutes = parts[1].parse::<u64>().map_err(|_| "Invalid minutes")?;
            let seconds = parts[2].parse::<u64>().map_err(|_| "Invalid seconds")?;
            Duration::from_secs(hours * 3600 + minutes * 60 + seconds)
        },
        _ => return Err("Invalid duration format"),
    };
    Ok(duration)
}

fn speak_message(message: &str) {
    let (command, args) = get_speak_command();

    let mut cmd = Command::new(command);
    cmd.args(args);
    cmd.arg(message);

    cmd.status().expect("Failed to execute speak command");
}

#[cfg(target_os = "macos")]
fn get_speak_command() -> (&'static str, Vec<&'static str>) {
    ("say", vec![])
}

#[cfg(target_os = "linux")]
fn get_speak_command() -> (&'static str, Vec<&'static str>) {
    ("spd-say", vec![])
}

#[cfg(target_os = "windows")]
fn get_speak_command() -> (&'static str, Vec<&'static str>) {
    // PowerShell command to use SpeechSynthesizer for speaking
    ("powershell", vec!["-Command", "Add-Type –AssemblyName System.speech; $speak = New-Object System.Speech.Synthesis.SpeechSynthesizer; $speak.Speak('"])
}

fn to_future_time_str(duration: Duration) -> String {
    let future_time: DateTime<Local> = Local::now() + duration;
    future_time.format("%l:%M:%S")
        .to_string()
}

fn main() {
    let matches = App::new("Speak After Delay")
        .version("0.1.0")
        .author("Nag")
        .about("Reminds you after a specified duration by speaking a message")
        .arg(Arg::new("estimate")
            .help("print nag time estimate based on duration provided")
            .short_alias('e')
            .long("estimate")
            .takes_value(false))
        .arg(Arg::with_name("duration")
            .help("Duration in hh:mm:ss, mm:ss, or ss format")
            .required(true)
            .index(1))
        .arg(Arg::with_name("message")
            .help("Message to speak when time has elapsed")
            .required(true)
            .index(2))
        .get_matches();

    let duration_str = matches.value_of("duration").unwrap();
    let duration = parse_duration(duration_str).expect("Failed to parse duration");
    let estimate = matches.is_present("estimate");
    let future_time = to_future_time_str(duration);

    println!("nagging @ {}", future_time.trim());

    if estimate {
        return;
    }

    let messages: Vec<&str> = matches.values_of("message").unwrap().collect();
    let concatenated_message = messages.join(" ");

    thread::sleep(duration);

    speak_message(&concatenated_message);
}