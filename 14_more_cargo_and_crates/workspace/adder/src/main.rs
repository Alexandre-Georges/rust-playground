use add_one;

// This can be run at the root of the workspace with this command : cargo run -p adder

// The rand crate is imported by the 2 projects but the workspace lock in the same version for them

// We can run tests for all projects by executing "cargo test" in the workspace folder
// Or run tests for a specific project with cargo test -p add-one
fn main() {
  let num = 10;
  println!(
    "Hello, world! {} plus one is {}!",
    num,
    add_one::add_one(num)
  );
}