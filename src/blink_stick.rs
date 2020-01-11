use std::time::Duration;
use std::{thread, time};

extern crate hidapi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct BlinkStick {
    device: hidapi::HidDevice,
}

impl BlinkStick {
    pub fn new() -> BlinkStick {
        let api = hidapi::HidApi::new().unwrap_or_else(|error| {
            panic!("Problem creating HID API interface: {:?}", error);
        });

        // Connect to device using its VID and PID
        let (vid, pid) = (0x20a0, 0x41e5);
        let device = api.open(vid, pid).unwrap_or_else(|error| {
            panic!("BlinkStick not found: {:?}", error);
        });
        BlinkStick { device }
    }

    fn set_color(&self, color: Color) {
        let Color { red, green, blue } = color;
        let buf = [0x1, red, green, blue];
        self.device.write(&buf).unwrap_or_default();
    }

    fn set_color_with_index(&self, channel: u8, index: u8, color: Color) {
        let Color { red, green, blue } = color;
        let buf = [0x5, channel, index, red, blue, green];
        self.device.write(&buf).unwrap_or_default();
    }

    pub fn blink(&self, color: Color, repeats: u128, delay: Duration) {
        for x in 1..repeats {
            if x > 0 {
                thread::sleep(delay);
            }
            self.set_color(color);
            thread::sleep(delay);
            self.switch_off();
        }
    }

    fn blink_nth(&self) {
        let buf = [0x5, 0, 2, 20, 20, 180];
        self.device.write(&buf).unwrap_or_default();
        let one_second = time::Duration::from_secs(1);
        thread::sleep(one_second);
        self.switch_off();
    }

    fn switch_off(&self) {
        let buf = [0x1, 0, 0, 0];
        self.device.write(&buf).unwrap_or_default();
        for number in 1..6 {
            let buf = [0x5, 0, number, 0, 0, 0];
            self.device.write(&buf).unwrap_or_default();
        }
    }
}
