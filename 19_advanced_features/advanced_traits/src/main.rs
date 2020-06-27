fn main() {
  {
    struct Counter {
      count: u32,
    }
    impl Counter {
      fn new() -> Counter {
        Counter { count: 0 }
      }
    }

    pub trait Iterator2 {
      type Item;
      fn next2(&mut self) -> Self::Item;
    }
    // This locks the type, we don't need to specify it but we can't change it.
    impl Iterator2 for Counter {
      type Item = u32;
      fn next2(&mut self) -> Self::Item {
        self.count = self.count + 1;
        self.count
      }
    };
    let mut c2 = Counter::new();
    println!("{}", c2.next2());
    println!("{}", c2.next2());

    pub trait Iterator3<T> {
      fn next3(&mut self) -> T;
    }
    impl Iterator3<u32> for Counter {
      fn next3(&mut self) -> u32 {
        self.count = self.count + 1;
        self.count
      }
    };
    // We need to define a new implementation for each concrete type
    let mut c3 = Counter::new();
    println!("{}", c3.next3());
    println!("{}", c3.next3());

    // I am not sure what it is all about, this works with generics:
    struct Ageo<T> {
      count: T,
    }
    impl<T> Ageo<T> {
      fn new(value: T) -> Ageo<T> {
        Ageo { count: value }
      }
    }
    pub trait GetCount<T> {
      fn get_count(&self) -> T;
    }
    impl<T : Copy> GetCount<T> for Ageo<T> {
      fn get_count(&self) -> T {
        self.count
      }
    };

    let a = Ageo::new(0);
    println!("{}", a.get_count());
  }

  // I think I missed the point but let's continue with operator overloading
  {
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
      x: i32,
      y: i32,
    }

    impl Add for Point {
      type Output = Point;

      fn add(self, other: Point) -> Point {
        Point {
          x: self.x + other.x,
          y: self.y + other.y,
        }
      }
    }

    assert_eq!(
      Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
      Point { x: 3, y: 3 }
    );
  }

  // The default generic type is like so (RHS : Right Hand Side which is the default type)
  {
    // RHS=Self means that the Add trait can be applied to the same type by default
    trait Add<RHS=Self> {
      type Output;
      fn add(self, rhs: RHS) -> Self::Output;
    }
  }

  // However we can also use Add with a different type
  {
    use std::ops::Add;

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
      type Output = Millimeters;

      // Here we add 2 different types
      fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
      }
    }
  }

  // We could have the same function for different types
  {
    trait Pilot {
      fn fly(&self);
    }
    trait Wizard {
      fn fly(&self);
    }
    struct Human;
    impl Pilot for Human {
      fn fly(&self) {
        println!("This is your captain speaking.");
      }
    }
    impl Wizard for Human {
      fn fly(&self) {
        println!("Up!");
      }
    }

    impl Human {
      fn fly(&self) {
        println!("*waving arms furiously*");
      }
    }

    let person = Human;
    // By default it runs the function on the defined type
    person.fly();

    // But we can make it execute the function for a different one
    Pilot::fly(&person);
    Wizard::fly(&person);
  }

  // Here Rust does not know which function to run so we need to specify the type
  {
    trait Animal {
      fn baby_name() -> String;
    }
    struct Dog;
    impl Dog {
      fn baby_name() -> String {
        String::from("Spot")
      }
    }

    impl Animal for Dog {
      fn baby_name() -> String {
        String::from("puppy")
      }
    }

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
  }

  // We can also make sure that the object has access to a function, this is a super trait
  {
    use std::fmt;

    // We make sure that we have access to the to_string function
    trait OutlinePrint: fmt::Display {
      fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
      }
    }

    struct Point {
      x: i32,
      y: i32,
    }

    impl fmt::Display for Point {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
      }
    }
  }

  // Here is how to implement traits that are outside the local crate in a non-local structure
  {
    /*
      This is not possible to do the following, traits and structs need to be in local crate.
      use std::fmt;
      impl fmt::Display for Vec<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
          write!(f, "[{}]", self.0.join(", "))
        }
      }
    */

    // However we can do that with a wrapper that takes a 1-tuple
    use std::fmt;
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
      }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
  }
}
