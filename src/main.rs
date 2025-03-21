mod alarm;
mod sounds;
mod repeat;
mod utils;

use alarm::Alarm;

fn main() {
    let alarm = Alarm::set_alarm();
    alarm.wait_until_alarm();
}
