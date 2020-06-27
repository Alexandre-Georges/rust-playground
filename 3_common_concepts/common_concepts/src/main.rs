fn main() {
  // By default variables are umutable
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  // Constants
  const MAX_POINTS: u32 = 100_000;
  println!("The value of MAX_POINTS is: {}", MAX_POINTS);

  const TWICE_MAX_POINTS: u32 = 100_000 * 2;
  println!("The value of TWICE_MAX_POINTS is: {}", TWICE_MAX_POINTS);

  // Shadowing of a variable (variable overridden), the type can be changed
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  println!("The value of x is: {}", x);

  // When parsing we need to have a type
  let guess: u32 = "42".parse().expect("Not a number!");
  println!("The value of guess is: {}", guess);

  /*
    Scalar types
    1. Integers

    Length  Signed  Unsigned
    8-bit   i8      u8
    16-bit  i16     u16
    32-bit	i32     u32
    64-bit	i64     u64
    128-bit	i128    u128
    arch    isize   usize

    Signed ones can be negative which divides by 2 the number of possible values
    i8 : -128 to 127
    u8: 0 to 255

    In debug mode, Rust will throw an error if it is outside the expected range.
    In release mode, Rust goes to the other end of the range and restarts from there.

    For arch it depends on the computer (64 bits for 64 bit computers for instance)

    Number literals   Example
    Decimal           98_222
    Hex               0xff
    Octal             0o77
    Binary            0b1111_0000
    Byte (u8 only)    b'A'

    2. Floating-point numbers : f32 and f64
    3. Booleans : bool
    4. Characters : char

    Compound types
    1. Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);
  */
  let tup = (500, 6.4, 1);

  let (x, y, z) = tup;

  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
  println!("The value of z is: {}", z);

  let x: (i32, f64, u8) = (500, 6.4, 1);

  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;

  println!("The value of five_hundred is: {}", five_hundred);
  println!("The value of six_point_four is: {}", six_point_four);
  println!("The value of one is: {}", one);

  /*
    2. Arrays

    let a = [1, 2, 3, 4, 5];

    To define an array, with 5 i32 elements :
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    Array containing 5 times the value 3 :
    let a = [3; 5];

    To get an element :
    let first = a[0];
  */

  // Functions
  fn another_function(x: i32) {
    println!("The value of x is: {}", x);
  }

  another_function(123);

  let x = 5;

  let y = {
      let x = 3;

      // No ; means it returns the value
      x + 1
  };

  println!("The value of x is 5: {}", x);
  println!("The value of y is 4: {}", y);

  fn five() -> i32 {
    5
  }

  println!("The execution of five is: {}", five());

  fn closure(f: fn()) {
    f();
  }
  fn closure_test() {
    println!("Closure")
  }
  closure(closure_test);


  let number = 6;
  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }


  let number = if true { 5 } else { 6 };
  println!("here is the number {}", number);


  let mut counter = 0;
  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {}", result);

  let mut number = 3;
  while number != 0 {
    println!("{}!", number);
    number -= 1;
  }

  let a = [10, 20, 30, 40, 50];
  for element in a.iter() {
    println!("the value is: {}", element);
  }

  for number in (1..4).rev() {
    println!("{}!", number);
  }
}
