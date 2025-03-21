use std::process::{Command, Stdio};
use chrono::NaiveTime;

pub struct Alarm {
    time: NaiveTime,
    sound: String,
    repeat: u32,
}

impl Alarm {
    pub fn new(time: &str, sound: &str, repeat: u32) -> Self {
        let time = NaiveTime::parse_from_str(time, "%H:%M").expect("Invalid time format");
        Self { time, sound: sound.to_string(), repeat }
    }

    pub fn start_in_background(&self) {
        let executable = std::env::current_exe()
            .expect("Failed to get current executable path")
            .parent()
            .expect("Failed to get executable directory")
            .join("alarm_process"); // Ensure correct binary path
    
        Command::new(executable)
            .arg(self.time.format("%H:%M").to_string())
            .arg(&self.sound)
            .arg(self.repeat.to_string())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start alarm in background");
    }
    
}
