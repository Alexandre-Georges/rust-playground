fn main() {
  // We can define our own types
  {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
  }

  // This is useful when we have long types
  {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    #[allow(unused_variables)]
    let f: Thunk = Box::new(|| println!("hi"));

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn takes_long_type(f: Thunk) {
    }
  }

  // Result uses a similar alias to not show the error in all signatures
  {
    #[allow(dead_code)]
    type Result<T> = std::result::Result<T, std::io::Error>;
  }

  // Rust has a never type, meaning the function never returns
  {
    #[allow(dead_code)]
    fn bar(guess: String) -> ! {
      #[allow(while_true)]
      loop {
        #[allow(unused_variables)]
        let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
        };
      }
    }
  }

  /*
    Types can be dynamically sized, like str.
    Rust has a trait Sized that determine if we know the size at compile time.
  */
  {
    // This code :
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn generic1<T>(t: T) {
    }

    // is understood as if it is :
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn generic2<T: Sized>(t: T) {
    }

    // If we want to have a generic function that works with unsized values
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn generic3<T: ?Sized>(t: &T) {
    }
  }
}
