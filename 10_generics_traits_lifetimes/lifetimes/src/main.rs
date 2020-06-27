fn main() {
  /*
    This implementation does not work because the compiler does not know the
    lifetime of the parameters

    fn longest(s1: &str, s2: &str) -> &str {
      if s1.len() > s2.len() {
        return s1;
      }
      s2
    }

    We need to specify the lifetime with 'a. They act like generics
    all types with 'a will have the shortest lifetime of all.
  */
  fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
      return s1;
    }
    s2
  }

  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("longest1 {}", result);

  /*
    This is also not valid because string2 has a shorter lifetime than string1.

    let string1 = String::from("long string is long");
    let result;
    {
      let string2 = String::from("xyz");
      result = longest(string1.as_str(), string2.as_str());
    }
    println!("longest2 {}", result);
  */

  // We can also have lifetimes in structures
  #[derive(Debug)]
  struct ImportantExcerpt<'a> {
    part: &'a str,
  }
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
      part: first_sentence,
  };
  println!("excerpt {:?}", i);

  /*
    The compiler uses 3 rules for lifetimes :
    - each parameter that is a reference gets its own lifetime parameter.
      In other words, a function with one parameter gets one lifetime parameter:
      fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters:
      fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

    - if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
      fn foo<'a>(x: &'a i32) -> &'a i32.

    - if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
      the lifetime of self is assigned to all output lifetime parameters.
      This third rule makes methods much nicer to read and write because fewer symbols are necessary.
  */
  impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
      3
    }
  }
  i.level();

  impl<'a> ImportantExcerpt<'a> {
    // The return has the same lifetime as self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
      println!("Attention please: {}", announcement);
      self.part
    }
  }
  i.announce_and_return_part("blah");

  // We can also have a static lifetime, meaning it is always there
  let s: &'static str = "I have a static lifetime.";
  println!("{}", s);


  // Everything together!
  use std::fmt::Display;

  fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
  ) -> &'a str
  where
    T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
      x
    } else {
      y
    }
  }

  println!("{}", longest_with_an_announcement(
    &String::from("aaa"),
    &String::from("bb"),
    String::from("cc")),
  )
}