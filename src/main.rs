mod alarm;
mod sounds;
mod repeat;
mod utils;

use alarm::Alarm;
use sounds::choose_sound;
use repeat::choose_repeat_mode;
use utils::prompt_user;

fn main() {
    let alarm_time = prompt_user("Enter alarm time (HH:MM):");
    let sound = choose_sound();
    let repeat_mode = choose_repeat_mode();
    
    let enter_reason = prompt_user("Do you want to enter a reason for the alarm? (yes/no):");
    let reason = if enter_reason.trim().eq_ignore_ascii_case("yes") {
        Some(prompt_user("Enter the reason for the alarm:"))
    } else {
        None
    };
    
    let alarm = Alarm::new(&alarm_time, &sound, repeat_mode, reason);
    alarm.start_in_background();
}
