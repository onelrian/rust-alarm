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
    
    let alarm = Alarm::new(&alarm_time, &sound, repeat_mode);
    alarm.start_in_background();
}
