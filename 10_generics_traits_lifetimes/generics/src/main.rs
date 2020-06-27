fn main() {
  // Reusing a generic type, the 2 points need to have the same type
  #[derive(Debug)]
  struct Point<T> {
    x: T,
    y: T,
  }
  let integer = Point { x: 5, y: 10 };
  let float = Point { x: 1.0, y: 4.0 };

  // With 2 generics we can have different types
  #[derive(Debug)]
  struct Point2<T, U> {
    x: T,
    y: U,
  }
  let integer_float = Point2 { x: 1.0, y: 4 };
  println!("integer_float {:#?}", integer_float);

  // Functions with generics
  impl<T> Point<T> {
    fn x(&self) -> &T {
      &self.x
    }
  }
  println!("integer.x() {:#?}", integer.x());

  // Function for one specific type
  impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
  }
  println!("float.distance_from_origin() {:#?}", float.distance_from_origin());

  // Mixing generics
  impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
      Point2 {
        x: self.x,
        y: other.y,
      }
    }
  }
  println!("float.distance_from_origin() {:#?}", integer_float.mixup(Point2 { x: "x", y: "y" }));
}