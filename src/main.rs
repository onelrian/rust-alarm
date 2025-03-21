mod alarm;
mod sounds;
mod repeat;
mod utils;

use alarm::set_alarm;
use sounds::choose_sound;
use repeat::choose_repeat_mode;
use utils::prompt_user;

fn main() {
    // Get alarm time from user
    let alarm_time = prompt_user("Enter alarm time (HH:MM):");

    // Select alarm sound
    let sound = choose_sound();

    // Choose repeat mode
    let repeat_mode = choose_repeat_mode();

    // Set and run alarm
    set_alarm(&alarm_time, &sound, repeat_mode);
}
