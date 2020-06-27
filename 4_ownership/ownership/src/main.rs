fn main() {
  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("{}", s);

  /*
    This does not work, it is a literal that can not be changed
    let mut s = "hello";
    s += "thing";
  */

  // Rust drops variables when they get out of scope

  // s1 becomes invalid after it is assigned to s2
  let s1 = String::from("hello");
  let s2 = s1;
  // println!("{}, world!", s1);
  println!("{}, world!", s2);


  // But we can clone
  let s1 = String::from("hello");
  let s2 = s1.clone();
  println!("s1 = {}, s2 = {}", s1, s2);

  fn test1() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

  } // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

  fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
  } // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

  fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
  } // Here, some_integer goes out of scope. Nothing special happens.

  test1();

  fn test2() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    println!("{} {}", s1, s3);
  } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
    // moved, so nothing happens. s1 goes out of scope and is dropped.

  fn gives_ownership() -> String {          // gives_ownership will move its
                                            // return value into the function
                                            // that calls it

    let some_string = String::from("hello");// some_string comes into scope

    some_string                             // some_string is returned and
                                            // moves out to the calling
                                            // function
  }

  // takes_and_gives_back will take a String and return one
  fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                        // scope
      a_string  // a_string is returned and moves out to the calling function
  }

  test2();

  // To keep ownership, we can use references: &
  fn test3() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
  }

  // This function does not take ownership of s and s can not be modified inside
  fn calculate_length(s: &String) -> usize {
    s.len()
  }

  test3();

  // We can still modify the referenced value with mut
  fn test4() {
    let mut s = String::from("hello");
    change(&mut s);
  }

  fn change(some_string: &mut String) {
    some_string.push_str(", world");
  }
  test4();

  // Slice function
  let s = String::from("hello world");
  let hello = &s[0..5];
  let world = &s[6..11];
  println!("{} {}", hello, world);

  let s = String::from("hello");
  let len = s.len();
  // Those 2 slices are the same
  let slice = &s[3..len];
  println!("{}", slice);
  let slice = &s[3..];
  println!("{}", slice);

  let s = String::from("hello");
  let len = s.len();
  // Same here they are the same
  let slice = &s[0..len];
  println!("{}", slice);
  let slice = &s[..];
  println!("{}", slice);
}
