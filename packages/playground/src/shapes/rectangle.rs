use std::fmt;

pub struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  pub fn new(width: u32, height: u32) -> Rectangle {
      Self {
          width,
          height
      }
  }
  pub fn area(&self) -> u64 {
      (self.width * self.height).into()
  }

  pub fn perimeter(&self) -> u64 {
      (2*self.width + 2*self.height).into()
  }
  
  pub fn can_fit(&self, rect: &Rectangle) -> bool {
      self.width > rect.width && self.height > rect.height
      || self.width > rect.height && self.height > rect.width
  }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle {{ W={}, H={} }}", self.width, self.height)
    }
}

#[cfg(test)]
mod rectangle_dimension_tests {
  use super::*;

  #[test]
  fn correct_area() {
      let rect = Rectangle::new(4, 8);
      assert_eq!(rect.area(), 32);
  }

  #[test]
  fn correct_perimeter() {
      let rect = Rectangle::new(4, 8);
      assert_eq!(rect.perimeter(), 24);
  }

  #[test]
  fn larger_can_fit_smaller() {
      let larger = Rectangle::new(4,8);
      let smaller = Rectangle::new(3,7);
      assert!(larger.can_fit(&smaller));
  }

  #[test]
  fn larger_can_fit_smaller_rotated() {
      let larger = Rectangle::new(4,8);
      let smaller = Rectangle::new(7,3);
      assert!(larger.can_fit(&smaller));
  }

  #[test]
  fn smaller_cannot_fit_larger() {
      let larger = Rectangle::new(4,8);
      let smaller = Rectangle::new(7,3);
      assert_eq!(smaller.can_fit(&larger), false);
  }
}
