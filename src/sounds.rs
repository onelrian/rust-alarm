use std::fs;
use crate::utils::prompt_user;

pub fn choose_sound() -> String {
    let sound_files = match fs::read_dir("sounds") {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "wav"))
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<String>>(),
        Err(_) => {
            println!("⚠️ Error: 'sounds/' directory not found!");
            return String::new();
        }
    };

    if sound_files.is_empty() {
        println!("⚠️ No sound files found in 'sounds/' directory.");
        return String::new();
    }

    println!("\nAvailable sounds:");
    for (i, file) in sound_files.iter().enumerate() {
        println!("{}. {}", i + 1, file);
    }

    let choice = prompt_user(&format!("Select a sound (1-{}):", sound_files.len()));
    let index: usize = choice.parse().unwrap_or(1);

    sound_files.get(index - 1).cloned().unwrap_or_else(|| sound_files[0].clone())
}
