use chrono::{DateTime, Datelike, Duration as ChronoDuration, Local, NaiveDateTime, Timelike};
use clap::{App, Arg};
use std::process::Command;
use std::thread;
use std::time::Duration;

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

fn format_target_time(duration: Duration) -> String {
    let now = Local::now();
    let future_time: DateTime<Local> = now + duration;
    let formatted_future_time = if future_time.day0() == now.day0() {
        future_time.format("%l:%M%P").to_string()
    } else {
        future_time.format("%l:%M%P (tomorrow)").to_string()
    };

    formatted_future_time.trim().to_string()
}

fn duration_after(s: &str) -> Result<Duration, &'static str> {
    let parts: Vec<&str> = s.split(':').collect();
    let duration = match parts.len() {
        1 => {
            let minutes = parts[0].parse::<u64>().map_err(|_| "Invalid minutes")?;
            Duration::from_secs(minutes * 60)
        }
        2 => {
            let minutes = parts[0].parse::<u64>().map_err(|_| "Invalid minutes")?;
            let seconds = parts[1].parse::<u64>().map_err(|_| "Invalid seconds")?;
            Duration::from_secs(minutes * 60 + seconds)
        }
        3 => {
            let hours = parts[0].parse::<u64>().map_err(|_| "Invalid hours")?;
            let minutes = parts[1].parse::<u64>().map_err(|_| "Invalid minutes")?;
            let seconds = parts[2].parse::<u64>().map_err(|_| "Invalid seconds")?;
            Duration::from_secs(hours * 3600 + minutes * 60 + seconds)
        }
        _ => return Err("Invalid duration format"),
    };
    Ok(duration)
}

fn duration_at(s: &str) -> Result<Duration, &'static str> {
    let (provided_time_str, period) = if s.to_lowercase().ends_with("am") {
        (&s[..s.len() - 2], "am")
    } else if s.to_lowercase().ends_with("pm") {
        (&s[..s.len() - 2], "pm")
    } else {
        (s, "")
    };

    let mut parts: Vec<&str> = provided_time_str.split(':').collect();
    if parts.len() == 1 {
        parts.push("00")
    } else if parts.len() != 2 {
        return Err("Invalid time format");
    }

    let mut hour = parts[0].trim().parse::<u32>().map_err(|_| "Invalid hour")?;
    let minute = parts[1]
        .trim()
        .parse::<u32>()
        .map_err(|_| "Invalid minute")?;

    let current_time = Local::now().time();
    if period == "am" && hour == 12 {
        hour = 0;
    } else if (period == "pm" && hour != 12)
        || hour < current_time.hour()
        || (hour == current_time.hour() && minute < current_time.minute())
    {
        hour += 12;
    }

    let current_date = Local::now().format("%Y-%m-%d").to_string();

    let datetime_str = format!("{} {:02}:{:02}:00", current_date, hour, minute);

    let target_date_time = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| "Invalid time format")?;

    let duration = {
        if target_date_time.time() > current_time {
            target_date_time.time() - current_time
        } else {
            current_time - target_date_time.time() + ChronoDuration::hours(24)
        }
    };

    Ok(duration
        .to_std()
        .map_err(|_| "Failed to convert duration")?)
}

fn main() {
    let matches = App::new("Speak After Delay")
        .version("0.1.0")
        .author("Nag")
        .about("Reminds you after a specified duration by speaking a message")
        .arg(
            Arg::new("estimate")
                .help("print nag time estimate based on duration provided")
                .short_alias('e')
                .long("estimate")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("when")
                .help("When to nag")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("target")
                .help("Duration in hh:mm:ss, mm:ss, or ss format")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("message")
                .help("Message to speak when time has elapsed")
                .required(true)
                .index(3),
        )
        .get_matches();

    let when_str = matches.value_of("when").unwrap().trim().to_lowercase();
    let target_str = matches.value_of("target").unwrap();

    let duration = if when_str == "in" {
        duration_after(target_str).expect("Failed to parse duration")
    } else if when_str == "at" {
        duration_at(target_str).expect("Failed to parse time")
    } else {
        eprintln!("Invalid value for 'when'. Only 'in' and 'at' are supported.");
        return;
    };

    let estimate = matches.is_present("estimate");
    let target_time = format_target_time(duration);

    println!("{}", target_time);

    if estimate {
        return;
    }

    let messages: Vec<&str> = matches.values_of("message").unwrap().collect();
    let concatenated_message = messages.join(" ");

    thread::sleep(duration);
    
    let now = Local::now();
    let target_time = Local::now() + duration;
    // It is possible that the time has already passed by the time we wake up. If we notify the user
    // in such cases, it would be confusing. So we only notify if the target time is within the next
    // 30 seconds since that's a number that still seems reasonable from a ux perspective.
    if target_time - now < ChronoDuration::seconds(30) {
        speak_message(&concatenated_message);
    }
}
