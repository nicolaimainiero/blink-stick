use blink_stick::blink_stick::BlinkStick;
use blink_stick::blink_stick::Color;
use std::time;

fn main() {
    let one_hundred_millis = time::Duration::from_millis(100);

    let stick = BlinkStick::new();
    stick.blink(
        Color {
            red: 100,
            green: 0,
            blue: 0,
        },
        10,
        one_hundred_millis,
    );
}
