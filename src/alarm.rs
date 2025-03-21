use chrono::{Local, Timelike, Duration as ChronoDuration};
use std::io;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use crate::repeat::should_ring_today;

pub fn set_alarm(time: &str, sound: &str, repeat_mode: u32) {
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 2 {
        println!("⚠️ Invalid time format! Use HH:MM.");
        return;
    }

    let (alarm_hour, alarm_minute): (u32, u32) = match (parts[0].parse(), parts[1].parse()) {
        (Ok(h), Ok(m)) if h < 24 && m < 60 => (h, m),
        _ => {
            println!("⚠️ Invalid time input!");
            return;
        }
    };

    loop {
        let now = Local::now();
        let mut alarm_time = now
            .with_hour(alarm_hour)
            .unwrap()
            .with_minute(alarm_minute)
            .unwrap()
            .with_second(0)
            .unwrap();

        // ✅ Fix: Convert DateTime<Local> to NaiveDate before passing
        if !should_ring_today(repeat_mode, &now.date_naive()) {
            println!("⏭️ Skipping today...");
            sleep(Duration::from_secs(60));
            continue;
        }

        if alarm_time < now {
            alarm_time = alarm_time + ChronoDuration::days(1);
        }

        let duration = (alarm_time - now).to_std().unwrap();
        println!("⏰ Alarm set for {}. Sleeping...", alarm_time.format("%H:%M"));
        sleep(duration);

        loop {
            println!("🔔 Alarm ringing! Playing sound...");
            let _ = Command::new("aplay").arg(sound).spawn();

            // Stop or snooze
            println!("(S) Stop | (Z) Snooze (5 min)");
            let mut response = String::new();
            io::stdin().read_line(&mut response).unwrap();
            match response.trim().to_lowercase().as_str() {
                "s" => {
                    println!("✅ Alarm stopped.");
                    break;
                }
                "z" => {
                    println!("⏳ Snoozing for 5 minutes...");
                    sleep(Duration::from_secs(300)); // Snooze for 5 minutes
                }
                _ => println!("⚠️ Invalid option!"),
            }
        }

        if repeat_mode == 1 {
            break;
        }
    }
}
