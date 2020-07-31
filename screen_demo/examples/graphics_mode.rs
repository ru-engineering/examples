extern crate linux_embedded_hal as hal;

use std::time::{Duration,Instant};

use embedded_graphics::{
    fonts::{Font6x8,Font6x6, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
    primitives::Line,
    primitives::Circle,
    primitives::Triangle,
    primitives::Rectangle,
    style::PrimitiveStyle,
};
use hal::I2cdev;
use ssd1306::{mode::GraphicsMode, Builder};

fn main(){
let i2c = I2cdev::new("/dev/i2c-1").unwrap();
let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

disp.init().unwrap();

let text_style2 = TextStyleBuilder::new(Font6x8)
    .text_color(BinaryColor::On)
    .build();
let text_style1 = TextStyleBuilder::new(Font6x6)
    .text_color(BinaryColor::On)
    .build();

let l = Line::new(Point::new(0, 0), Point::new(128, 15))
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off,8));


let t2 = Text::new("This display module\n is now fully active", Point::zero())
    .into_styled(text_style2);
let t1 = Text::new("The loading was fake btw", Point::new(0, 55))
    .into_styled(text_style1);
Text::new("Loading", Point::new(16,16))
        .into_styled(text_style2)
        .draw(&mut disp).unwrap();
disp.flush().unwrap();
for i in 1..64 {
    Line::new(Point::new(16, 32), Point::new(16+i, 32))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On,8))
        .draw(&mut disp).unwrap();
    let timer = Instant::now();
    while timer.elapsed() < Duration::from_millis(100){
        l.draw(&mut disp).unwrap();
        Text::new(&((timer.elapsed().as_millis()).to_string()), Point::zero())
            .into_styled(text_style1)
            .draw(&mut disp).unwrap();
        disp.flush().unwrap();
    }
}
disp.clear();
disp.flush().unwrap();
t1.draw(&mut disp).unwrap();
t2.draw(&mut disp).unwrap();
disp.flush().unwrap();

Triangle::new(Point::new(32, 26), Point::new(16, 48), Point::new(48, 48))
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On,1))
    .draw(&mut disp).unwrap();

Circle::new(Point::new(64, 40), 10)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On,1))
        .draw(&mut disp).unwrap();

Rectangle::new(Point::new(80, 26), Point::new(120, 48))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On,1))
            .draw(&mut disp).unwrap();

disp.flush().unwrap();


}
