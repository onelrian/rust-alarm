use chrono::{Datelike, NaiveDate};
use crate::utils::prompt_user;

pub fn choose_repeat_mode() -> u32 {
    println!("\nRepeat mode:");
    println!("1. One-time");
    println!("2. Daily");
    println!("3. Weekly");
    println!("4. Custom days (e.g., Mon, Wed, Fri)");

    let choice = prompt_user("Select repeat mode (1-4):");
    let repeat_mode = choice.parse::<u32>().unwrap_or(1);

    if repeat_mode == 4 {
        println!("Enter days (Mon,Wed,Fri):");
        let custom_days_input = prompt_user("Days:");
        let custom_days: Vec<String> = custom_days_input
            .split(',')
            .map(|d| d.trim().to_lowercase())
            .collect();

        println!("Alarm will ring on {:?}", custom_days);
    }

    repeat_mode
}

pub fn should_ring_today(repeat_mode: u32, today: &NaiveDate) -> bool {
    match repeat_mode {
        1 => true, // One-time
        2 => true, // Daily
        3 => today.weekday().number_from_monday() == 1, // Weekly (Monday)
        4 => true, // Custom (To be handled)
        _ => false,
    }
}
