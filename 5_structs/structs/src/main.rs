fn main() {
  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
  }

  let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  println!("{}", user1.email);
  println!("{}", user1.username);
  println!("{}", user1.active);
  println!("{}", user1.sign_in_count);

  // To edit a struct, it needs to be mutable
  let mut user2 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
  };

  user2.email = String::from("anotheremail@example.com");

  println!("{}", user2.email);
  println!("{}", user2.username);
  println!("{}", user2.active);
  println!("{}", user2.sign_in_count);

  fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
  }

  let user3 = build_user(String::from("email"), String::from("username"));
  println!("{}", user3.email);
  println!("{}", user3.username);
  println!("{}", user3.active);
  println!("{}", user3.sign_in_count);

  // We can also create new structs based on an other one
  let user4 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
  };
  println!("{}", user4.email);
  println!("{}", user4.username);
  println!("{}", user4.active);
  println!("{}", user4.sign_in_count);

  // Tuple structs can be used when we don't want to have specific variable names
  struct Color(i32, i32, i32);
  let black = Color(0, 0, 0);
  println!("{}", black.0);

  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }

  let rect1 = Rectangle {
      width: 30,
      height: 50,
  };

  println!("rect1 is {:?}", rect1);
  // New line between each field
  println!("rect1 is {:#?}", rect1);

  // To define a method for the structure
  impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
  }
  println!("The area of the rectangle is {} square pixels.", rect1.area());

  fn area2(rect: &Rectangle) -> u32 {
    &rect.width * &rect.height
  }
  println!("The area of the rectangle is {} square pixels.", area2(&rect1));

  // We can have multiple impl blocks
  impl Rectangle {
    // Here is a constructor
    fn square(size: u32) -> Rectangle {
      Rectangle {
          width: size,
          height: size,
      }
    }
  }

  let square = Rectangle::square(10);
  println!("The area of the square is {} square pixels.", square.area());
}
