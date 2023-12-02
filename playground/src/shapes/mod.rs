mod rectangle;

use rectangle::Rectangle;

pub fn run() {
  let rectangle = Rectangle::new(3, 7);
  println!("{}", rectangle);
}