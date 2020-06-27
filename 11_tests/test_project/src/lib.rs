#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

pub fn add_two(a: i32) -> i32 {
  a + 2
}

pub fn greeting(name: &str) -> String {
  String::from("Hello!")
}

pub struct Guess { value: i32 }
impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }
    Guess { value }
  }
}

pub struct Guess2 { value: i32 }
impl Guess2 {
  pub fn new(value: i32) -> Guess {
    if value < 1 {
      panic!("Guess value must be greater than or equal to 1, got {}.", value);
    } else if value > 100 {
      panic!("Guess value must be less than or equal to 100, got {}.", value);
    }
    Guess { value }
  }
}

#[cfg(test)]
mod tests {

  // Run tests with : cargo test
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
  /*
    This makes the test fail
    #[test]
    fn another() {
      panic!("Make this test fail");
    }
  */

  // This allows us to import the Rectangle struct
  use super::*;

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };

    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };

    assert!(!smaller.can_hold(&larger));
  }

  #[test]
  fn it_adds_two() {
    assert_eq!(4, add_two(2));
  }

  // There is also : assert_ne!

  // For structures and enumerations, they will need to implement PartialEq to make them comparable (#[derive(PartialEq, Debug)])

  /*
    We can have custom messages
    #[test]
    fn greeting_contains_name() {
      let result = greeting("Carol");
      assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
    }
  */

  // We can test that a function should panic
  #[test]
  #[should_panic]
  fn greater_than_100() {
    Guess::new(200);
  }

  /*
    To look for a specific exception, we can match against the text
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_2() {
      Guess2::new(200);
    }
  */

  // Result can be used for tests
  #[test]
  fn it_works2() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }

  // "cargo test" compiles and run the binary, it executes tests in parallel
  // "cargo test --help" displays the option for testing
  // "cargo test -- --help" shows all the options after --

  // If we use println! in our code, we can see the output of passing tests with "cargo test -- --show-output"
  // Failing tests' output is automatically shown

  // To run specific test(s) "cargo test one_hundred" will run all tests that have a name which starts with "one_hundred"

  // We can ignore a test
  #[test]
  #[ignore]
  fn expensive_test() {
  }

  // To run ignored tests "cargo test -- --ignored"

  // Private functions (no pud) can be tested like public functions
}