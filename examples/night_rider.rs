use blink_stick::color::Color;
use blink_stick::BlinkStick;
use std::time;

fn main() {
  let one_hundred_millis = time::Duration::from_millis(100);

  let stick = BlinkStick::new();
  //stick.blink(Color::new(100, 0, 0), 10, one_hundred_millis);
  loop {
    for i in 0..=7 {
      stick.blink_nth(i, Color::new(100, 0, 0), 1, one_hundred_millis);
    }
    for i in 0..=7 {
      stick.blink_nth(7 - i, Color::new(100, 0, 0), 1, one_hundred_millis);
    }
  }
}
