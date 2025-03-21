use std::io::{self, Write};
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

pub struct Alarm {
    pub sound_path: String,
    pub repeat: bool,
}

impl Alarm {
    pub fn ring(&self) {
        println!("🔔 Alarm ringing! Playing sound...");

        // Start playing the sound and keep track of the process
        let mut sound_process: Child = Command::new("aplay")
            .arg(&self.sound_path)
            .spawn()
            .expect("⚠️ Failed to play sound!");

        loop {
            // Give the user control options
            println!("\nOptions: [s] Snooze | [x] Stop");
            print!("Enter choice: ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim().to_lowercase();

            match choice.as_str() {
                "s" => {
                    println!("😴 Snoozing for 5 minutes...");
                    sound_process.kill().expect("⚠️ Failed to stop sound!");
                    sleep(Duration::from_secs(300)); // Snooze for 5 minutes
                    self.ring(); // Restart alarm after snooze
                }
                "x" => {
                    println!("✅ Alarm stopped.");
                    sound_process.kill().expect("⚠️ Failed to stop sound!");
                    break;
                }
                _ => println!("⚠️ Invalid option. Try again."),
            }
        }
    }
}
