use std::env;
use std::process;
use io_project_with_iterators::Config;

// CASE_INSENSITIVE=true cargo run to poem.txt

/*
  cargo run p1 p2
  gives us
  ["target/debug/io-project", "p1", "p2"]
*/
fn main() {
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });

  // We use this instead of unwrap_or_else because we don't care about the Ok
  if let Err(e) = io_project_with_iterators::run(config) {
    eprintln!("Application error: {}", e);
    process::exit(1);
  }
}
