use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::error::Error;
use std::net::IpAddr;

fn main() {
  // The file does not exist so we get an error from the result
  let f = File::open("hello.txt");
  let f = match f {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };


  // We can catch specific errors
  let f = File::open("hello.txt");
  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error)
      }
    },
  };

  // We could also write it like that:
  let f = File::open("hello.txt").unwrap_or_else( |error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else( |error| {
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  });

  // This is less verbose, it returns the result of the Ok or calls panick if there is an error
  let f = File::open("hello.txt").unwrap();
}

// To throw errors
fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}

// Another implementation with ?
// ? returns directly the error when there is one
// ? converts the error that gets thrown into the error of the signature (if possible)
// ? can be used in functions that return a Result or an Option
fn read_username_from_file2() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}

// Even less verbose
fn read_username_from_file3() -> Result<String, io::Error> {
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}

// Rust provides a function to do that
fn read_username_from_file4() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}

// The main function can also be done like so
fn main2() -> Result<(), Box<dyn Error>> {
  let f = File::open("hello.txt")?;

  Ok(())
}

// When we are sure that calling a function will never return an error we can use unwrap
// It unwrap the Ok and no error is
fn when_we_know() {
  let home: IpAddr = "127.0.0.1".parse().unwrap();
}
