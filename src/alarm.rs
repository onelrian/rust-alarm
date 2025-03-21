use chrono::{Local, NaiveTime, Timelike};
use std::process::{Command, Child};
use std::thread::sleep;
use std::time::Duration;
use crate::sounds::choose_sound;
use crate::repeat::choose_repeat_mode;
use crate::utils::prompt_user;

pub struct Alarm {
    pub time: NaiveTime,
    pub sound: String,
    pub repeat: u32,
}

impl Alarm {
    pub fn set_alarm() -> Self {
        let time_input = prompt_user("Enter alarm time (HH:MM):");
        let alarm_time = NaiveTime::parse_from_str(&time_input, "%H:%M").expect("Invalid time format!");

        let selected_sound = choose_sound();
        let repeat_mode = choose_repeat_mode();

        Self {
            time: alarm_time,
            sound: selected_sound,
            repeat: repeat_mode,
        }
    }

    pub fn wait_until_alarm(&self) {
        loop {
            let now = Local::now().time();

            if now.hour() == self.time.hour() && now.minute() == self.time.minute() {
                self.ring_alarm();

                // Stop if it's a one-time alarm
                if self.repeat == 1 {
                    break;
                }
            }
            sleep(Duration::from_secs(30));
        }
    }

    fn ring_alarm(&self) {
        println!("🔔 Alarm ringing! Playing sound...");
        let mut child: Child = Command::new("aplay")
            .arg(&self.sound)
            .spawn()
            .expect("Failed to play sound");

        loop {
            let user_input = prompt_user("Press 's' to stop, 'z' to snooze for 5 minutes:");
            match user_input.as_str() {
                "s" => {
                    println!("⏹️ Stopping alarm.");
                    let _ = child.kill();
                    break;
                }
                "z" => {
                    println!("😴 Snoozing for 5 minutes...");
                    let _ = child.kill();
                    sleep(Duration::from_secs(300));
                    self.ring_alarm();
                    break;
                }
                _ => println!("⚠️ Invalid input. Try again."),
            }
        }
    }
}
