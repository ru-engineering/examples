use core::fmt::Write;
use ssd1306::{mode::TerminalMode, prelude::*, Builder};
extern crate linux_embedded_hal as hal;

use hal::I2cdev;
use std::time::Duration;
use std::thread;


fn main() {
let i2c = I2cdev::new("/dev/i2c-1").unwrap();
let mut disp: TerminalMode<_> = Builder::new().connect_i2c(i2c).into();
disp.init().unwrap();
let _ = disp.clear();

// Spam some characters to the display
    disp.write_str("this is just a test 123\n");
    disp.write_fmt(format_args!("{} and {}\n",1,"ok"));
    for s in 1..100{
        disp.write_fmt(format_args!("{} \t",s));
        thread::sleep(Duration::from_millis(500));
    }
}
