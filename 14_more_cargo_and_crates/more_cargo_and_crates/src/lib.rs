/*
  Cargo has 2 profiles when building :
  - cargo build : dev (faster to build)
  - cargo build --release : release (slower to build but more optimized)

  The default settings can be overridden in the Cargo.toml file :
  [profile.dev]
  opt-level = 1

  We can also create documentation via specific comments and generate the doc with :
  cargo doc
  or cargo doc --open
  to open it in the browser right away

  There are a few sections that can be used with # in front of them.
*/

//! # More cargo and creates
//!
//! Crate related comments

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// # Panics
///
/// It can panic if blah blah
///
/// # Errors
///
/// An error can be returned if blah blah
///
/// # Safety
///
/// This is unsafe because why not
pub fn add_one(x: i32) -> i32 {
	x + 1
}

// Modules can also be re-exported to make them easier to use (no long use blah::blah::thing) :
pub mod kinds {
  // Here we would need to import it like so: use art::kinds::Color;
  #[derive(Debug)]
  pub enum Color {
    Red,
  }
}

pub mod utils {
  use super::kinds::Color;

  pub fn get() -> Color {
    Color::Red
  }
}

// This re-exports the functions
pub use self::kinds::Color;
pub use self::utils::get;

// Then we can import them like that : use art::Color;