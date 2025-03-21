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
        let _custom_days = prompt_user("Days:");
        // You can store custom days for further handling
    }

    repeat_mode
}
