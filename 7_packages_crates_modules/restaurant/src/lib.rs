// We are at the root of the "crate"

// First we define a module
mod front_of_house {
  // We can have child modules
  pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
  }

  mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
  }
}

// By default everything is private, we need to expose modules and methods to make them accessible

// We publish a public function
pub fn eat_at_restaurant() {
  // Modules' methods can be called with an absolute path
  crate::front_of_house::hosting::add_to_waitlist();

  // or a relative path
  front_of_house::hosting::add_to_waitlist();
}

// We can also call a parent function
fn serve_order() {}

mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    // Super is equivalent to ..
    super::serve_order();
  }

  fn cook_order() {}
}

mod back_of_house2 {
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    // The constructor is required because we have a private field
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }
}

pub fn eat_at_restaurant2() {
  let mut meal = back_of_house2::Breakfast::summer("Rye");

  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  // The next line won't compile if we uncomment it; we're not allowed
  // to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");
}

// All fields are public, no need to have a constructor
mod back_of_house3 {
  pub enum Appetizer {
    Soup,
    Salad,
  }
}

pub fn eat_at_restaurant3() {
  let order1 = back_of_house3::Appetizer::Soup;
  let order2 = back_of_house3::Appetizer::Salad;
}


// We can "import" in a variable a module
mod front_of_house2 {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

use crate::front_of_house2::hosting;

pub fn eat_at_restaurant4() {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}

// We can get access to the function
mod front_of_house3 {
  pub mod hosting {
      pub fn add_to_waitlist() {}
  }
}

use crate::front_of_house3::hosting::add_to_waitlist;

pub fn eat_at_restaurant5() {
  add_to_waitlist();
  add_to_waitlist();
  add_to_waitlist();
}

// We can rename the "package"
use std::fmt::Result;
use std::io::Result as IoResult;

// To rename an import and export it
mod front_of_house4 {
  pub mod hosting2 {
      pub fn add_to_waitlist() {}
  }
}

pub use crate::front_of_house4::hosting2;

pub fn eat_at_restaurant6() {
  hosting2::add_to_waitlist();
  hosting2::add_to_waitlist();
  hosting2::add_to_waitlist();
}

// To import a crate from crates.io, add `rand = "0.5.5"` to the Cargo.toml file
use rand::Rng;

println!("{}", rand::thread_rng().gen_range(1, 101));

// Imports from the same crate can be grouped
use std::{cmp::Ordering, io};

// Those lines are equivalent to the other one below
use std::io;
use std::io::Write;

use std::io::{self, Write};

// And to bring everything from the package
use std::collections::*;
