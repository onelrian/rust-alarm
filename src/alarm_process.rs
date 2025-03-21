use std::env;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use chrono::{Local, NaiveTime, Timelike};

fn main() {
    let args: Vec<String> = env::args().collect();
    let alarm_time = NaiveTime::parse_from_str(&args[1], "%H:%M").expect("Invalid time format");
    let sound = &args[2];
    let repeat = args[3].parse::<u32>().unwrap();

    loop {
        let now = Local::now().time();
        if now.hour() == alarm_time.hour() && now.minute() == alarm_time.minute() {
            let _ = Command::new("notify-send")
                .arg("Alarm Ringing! Click to stop or snooze")
                .spawn();
            
            let mut child = Command::new("aplay").arg(sound).spawn().expect("Failed to play sound");

            if let Ok(status) = child.wait() {
                if status.success() {
                    if repeat == 1 {
                        break;
                    }
                }
            }
        }
        sleep(Duration::from_secs(30));
    }
}
