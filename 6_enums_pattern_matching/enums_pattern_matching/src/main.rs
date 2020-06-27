fn main() {
  #[derive(Debug)]
  enum IpAddrKind {
    V4,
    V6,
  }

  #[derive(Debug)]
  struct IpAddr {
    kind: IpAddrKind,
    address: String,
  }

  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };

  println!("Home {:#?}", home);
  println!("Loopback {:#?}", loopback);

  // This is more concise
  #[derive(Debug)]
  enum IpAddr2 {
    V4(String),
    V6(String),
  }

  let home = IpAddr2::V4(String::from("127.0.0.1"));
  let loopback = IpAddr2::V6(String::from("::1"));

  println!("Home {:#?}", home);
  println!("Loopback {:#?}", loopback);


  // We can also have different types of params
  #[derive(Debug)]
  enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
  }

  let home = IpAddr3::V4(127, 0, 0, 1);
  let loopback = IpAddr3::V6(String::from("::1"));

  println!("Home {:#?}", home);
  println!("Loopback {:#?}", loopback);


  // Another example
  #[derive(Debug)]
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  impl Message {
    fn call(&self) {
      println!("Message {:?}", self);
    }
  }

  Message::Quit.call();
  (Message::Move { x: 1, y: 2 } ).call();
  Message::Write(String::from("hello")).call();
  Message::ChangeColor(1, 2, 3).call();


  // Rust does not have null like in other languages but it has a structure that expresses the concept
  let some_number = Some(5);
  let some_string = Some("a string");
  let absent_number: Option<i32> = None;

  // Option can be Some or None
  println!("Message {:?}", some_number);
  println!("Message {:?}", some_string);
  println!("Message {:?}", absent_number);


  // We then need a pattern matcher for each option
  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
  }

  fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
          println!("Lucky penny!");
          1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
  }

  println!("Value of a Penny {}", value_in_cents(Coin::Penny));
  println!("Value of a Nickel {}", value_in_cents(Coin::Nickel));
  println!("Value of a Dime {}", value_in_cents(Coin::Dime));
  println!("Value of a Quarter {}", value_in_cents(Coin::Quarter));


  #[derive(Debug)] // so we can inspect the state in a minute
  enum UsState {
    Alabama,
    Alaska,
  }

  enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
  }

  fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
      Coin2::Penny => 1,
      Coin2::Nickel => 5,
      Coin2::Dime => 10,
      Coin2::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        match state {
          UsState::Alabama => println!("Sweet Home Alabama"),
          UsState::Alaska => println!("Brrr brrr"),
        }
        25
      }
    }
  }

  println!("Value of a Penny {}", value_in_cents2(Coin2::Penny));
  println!("Value of a Nickel {}", value_in_cents2(Coin2::Nickel));
  println!("Value of a Dime {}", value_in_cents2(Coin2::Dime));
  println!("Value of a Quarter {}", value_in_cents2(Coin2::Quarter(UsState::Alabama)));
  println!("Value of a Quarter {}", value_in_cents2(Coin2::Quarter(UsState::Alaska)));

  // For math now
  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
  }

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
  println!("five {:?}", five);
  println!("six {:?}", six);
  println!("none {:?}", none);


  // There is also a wildcard
  let some_u8_value = 0u8;
  match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
  };


  // The wildcard does not need to be written, for instance this:
  let some_u8_value = Some(0u8);
  match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
  }

  // can be written like so:
  if let Some(3) = some_u8_value {
    println!("three");
  }


  let coin = Coin2::Penny;
  // Those 2 pieces of code are identical
  let mut count = 0;
  match coin {
    Coin2::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
  }
  println!("count {}", count);

  let coin = Coin2::Penny;
  let mut count = 0;
  if let Coin2::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
  } else {
    count += 1;
  }
  println!("count {}", count);
}
