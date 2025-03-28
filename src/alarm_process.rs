use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use chrono::{Local, NaiveTime, Timelike};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 4 {
        eprintln!("Usage: {} <HH:MM> <sound_file> <repeat> [reason]", args[0]);
        return;
    }

    let alarm_time = NaiveTime::parse_from_str(&args[1], "%H:%M").expect("Invalid time format");
    let sound = &args[2];
    let repeat = args[3].parse::<u32>().unwrap();
    let reason = args.get(4).cloned().unwrap_or_else(|| "No reason provided".to_string());

    loop {
        let now = Local::now().time();
        if now.hour() == alarm_time.hour() && now.minute() == alarm_time.minute() {
            let mut child = Command::new("aplay")
                .arg(sound)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to play sound");

            println!("⏰ Alarm is ringing!");
            println!("📢 Reason: {}", reason);

            // Properly formatted notify-send command with reason
            let _ = Command::new("notify-send")
                .arg("⏰ Alarm Ringing!")
                .arg(format!("📢 Reason: {}\nClick to Stop or Snooze", reason)) // Ensure correct formatting
                .arg("--action=stop=Stop")
                .arg("--action=snooze=Snooze")
                .spawn();

            // Wait for user input via `dbus-monitor`
            let output = Command::new("dbus-monitor")
                .arg("interface='org.freedesktop.Notifications', member='ActionInvoked'")
                .output()
                .expect("Failed to monitor notifications");

            let output_str = String::from_utf8_lossy(&output.stdout);
            
            if output_str.contains("stop") {
                let _ = child.kill();
                println!("⏹️ Alarm Stopped.");
                break;
            } else if output_str.contains("snooze") {
                let _ = child.kill();
                println!("😴 Snoozing for 5 minutes...");
                sleep(Duration::from_secs(300)); // Snooze for 5 minutes
                continue;
            }
        }

        sleep(Duration::from_secs(30)); // Check every 30 seconds
    }
}
