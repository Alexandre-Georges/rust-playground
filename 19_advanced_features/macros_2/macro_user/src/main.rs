// Procedural macros need to be in their own crate
use ageo_macro::HelloMacro;
use ageo_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
  Pancakes::hello_macro();
}