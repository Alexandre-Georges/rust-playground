fn main() {

  // We can use matchers and unwrap a value in if/else expressions
  {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
      println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
      println!("Tuesday is green day!");
    } else if let Ok(age) = age {
      if age > 30 {
        println!("Using purple as the background color");
      } else {
        println!("Using orange as the background color");
      }
    } else {
      println!("Using blue as the background color");
    }
  }

  // Same with a loop
  {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
      println!("{}", top);
    }
  }

  // We can also get the index when we iterate
  {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
      println!("{} is at index {}", value, index);
    }
  }

  // Pattern matching and unwrapping the value
  {
    let some_option_value = Some(1);
    if let Some(x) = some_option_value {
      println!("Some {}", x);
    }
  }

  // Another example, variables get shadowed
  {
    let x = Some(5);
    let y = 10;

    match x {
      Some(50) => println!("Got 50"),
      Some(y) => println!("Matched, y = {:?}", y),
      _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
  }

  // With an or
  {
    let x = 1;

    match x {
      1 | 2 => println!("one or two"),
      3 => println!("three"),
      _ => println!("anything"),
    }
  }

  // With a range
  {
    let x = 5;

    match x {
      1..=5 => println!("one through five"),
      _ => println!("something else"),
    }
  }

  // It works also with chars
  {
    let x = 'c';

    match x {
      'a'..='j' => println!("early ASCII letter"),
      'k'..='z' => println!("late ASCII letter"),
      _ => println!("something else"),
    }
  }

  // Like in Javascript, we can destructure objects
  {
    struct Point {
      x: i32,
      y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
  }

  // Combining destructuring with patterns
  {
    struct Point {
      x: i32,
      y: i32,
    }

    let p = Point { x: 0, y: 7 };

    match p {
      Point { x, y: 0 } => println!("On the x axis at {}", x),
      Point { x: 0, y } => println!("On the y axis at {}", y),
      Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
  }

  // This works also with enums
  {
    #[allow(dead_code)]
    enum Message {
      Quit,
      ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
      Message::Quit => {
        println!("The Quit variant has no data to destructure.")
      }
      Message::ChangeColor(r, g, b) => println!(
        "Change the color to red {}, green {}, and blue {}",
        r, g, b
      ),
    }
  }

  // We can have nested patterns
  {
    #[allow(dead_code)]
    enum Color {
      Rgb(i32, i32, i32),
      Hsv(i32, i32, i32),
    }

    #[allow(dead_code)]
    enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
      Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
        "Change the color to red {}, green {}, and blue {}",
        r, g, b
      ),
      Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
        "Change the color to hue {}, saturation {}, and value {}",
        h, s, v
      ),
      _ => (),
    }
  }

  // Another one
  {
    struct Point {
      x: i32,
      y: i32,
    }
    #[allow(unused_variables)]
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
  }

  // Ignoring a value
  {
    fn foo(_: i32, y: i32) {
      println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);
  }

  // Ignoring a value in a matcher
  {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
      (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
      }
      _ => {
        setting_value = new_setting_value;
      }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
      (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
      }
    }
  }

  // Ignoring an unused variable
  {
    let _x = 5;
  }

  // Ignoring parts of the value
  {
    #[allow(dead_code)]
    struct Point {
      x: i32,
      y: i32,
      z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
      // We just want x
      Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
      (first, .., last) => {
        println!("Some numbers: {}, {}", first, last);
      }
    }
  }

  // We can have an if in a matcher, it will execute just one of the options
  {
    let num = Some(4);
    match num {
      Some(x) if x < 5 => println!("less than five: {}", x),
      Some(x) => println!("{}", x),
      None => (),
    }
  }

  // @ also validates that value
  {
    enum Message {
      Hello { id: i32 },
    }

    let msg = Message::Hello { id: 8 };

    match msg {
      Message::Hello {
        // id is in the [3,7] range
        id: id_variable @ 3..=7,
      } => println!("Found an id in range: {}", id_variable),
      Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
      }
      Message::Hello { id } => println!("Found some other id: {}", id),
    }
  }
}
