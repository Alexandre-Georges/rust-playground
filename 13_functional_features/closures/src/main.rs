use std::thread;
use std::time::Duration;

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
  generate_workout2(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
  // This is a closure, closures have access to the current scope
  let expensive_closure = |num| {
    println!("expensive_closure: calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  };

  /*
    We can also define types for it like so
    let expensive_closure = |num: u32| -> u32 {
      println!("calculating slowly...");
      thread::sleep(Duration::from_secs(2));
      num
    };
  */

  if intensity < 25 {
    println!("generate_workout: Today, do {} pushups!", expensive_closure(intensity));
    println!("generate_workout: Next, do {} situps!", expensive_closure(intensity));
  } else {
    if random_number == 3 {
      println!("generate_workout: Take a break today! Remember to stay hydrated!");
    } else {
      println!("generate_workout: Today, run for {} minutes!", expensive_closure(intensity));
    }
  }
}

fn generate_workout2(intensity: u32, random_number: u32) {
  let mut expensive_result = Cacher::new(|num| {
    println!("expensive_result2: calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  });

  if intensity < 25 {
    println!("generate_workout2: Today, do {} pushups!", expensive_result.value(intensity));
    println!("generate_workout2: Next, do {} situps!", expensive_result.value(intensity));
  } else {
    if random_number == 3 {
      println!("generate_workout2: Take a break today! Remember to stay hydrated!");
    } else {
      println!("generate_workout2: Today, run for {} minutes!", expensive_result.value(intensity));
    }
  }
}

// This is a memoization class
struct Cacher<T>
where T: Fn(u32) -> u32 {
  calculation: T,
  value: Option<u32>,
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32 {
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

/*
  There are 3 types of closures :
  - FnOnce : the closure takes ownership of the variables it captures from the scope
  - FnMut : the closure can change the values of the variables it borrows
  - Fn : values are immutable
*/

// To force a closure to take ownership of variables
fn move_ownership() {
  let x = vec![1, 2, 3];
  let equal_to_x = move |z| z == x;

  println!("equal_to_x {}", equal_to_x(vec![1]))
  // x is inaccessible here, its ownership got moved to the closure
}
