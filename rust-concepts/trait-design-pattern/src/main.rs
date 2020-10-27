// main.rs
// https://stackoverflow.com/questions/62965609/definition-of-traits-in-a-separate-file
// Modified by Joseph T. Foley <foley AT RU.IS> with help from shivanandvp from Rust Community Discord channel on 2020-10-27

mod geometry;
use geometry::circle::Circle;
use geometry::rect::Rect;
use geometry::traits::Area;

fn main() {
    let rect = Rect {
        width: 1.0,
        length: 2.0,
    };
    let circle = Circle { radius: 2.3 };

    println!("{}", rect.area());
    println!("{}", circle.area());
}
