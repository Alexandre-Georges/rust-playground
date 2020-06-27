fn main() {
  {
    // The object is exposed but not its properties
    pub struct AveragedCollection {
      list: Vec<i32>,
      average: f64,
    }
    impl AveragedCollection {
      pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
      }

      pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
          Some(value) => {
            self.update_average();
            Some(value)
          }
          None => None,
        }
      }

      pub fn average(&self) -> f64 {
        self.average
      }

      fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
      }
    }
  }
  {

    // Rust does not have inheritance but can behave similarly with traits' defaults
    pub trait Summary {
      fn summarize(&self) -> String {
        String::from("(Read more...)")
      }
    }

    // For the polymorphism aspect, Rust has generics.
  }

  {
    // Example using traits, at run time?
    pub trait Draw {
      fn draw(&self);
    }

    pub struct Screen {
      // This provides a size
      pub components: Vec<Box<dyn Draw>>,
    }
    impl Screen {
      pub fn run(&self) {
        for component in self.components.iter() {
          component.draw();
        }
      }
    }

    // Alternative implementation with generics, that looks much better to me. Works at compile time.
    pub struct Screen2<T: Draw> {
      pub components: Vec<T>,
    }
    impl<T> Screen2<T> where T: Draw {
      pub fn run(&self) {
        for component in self.components.iter() {
          component.draw();
        }
      }
    }

    pub struct Button {
      pub width: u32,
      pub height: u32,
      pub label: String,
    }
    impl Draw for Button {
      fn draw(&self) {
        println!("Draw Button");
      }
    }

    struct SelectBox {
      width: u32,
      height: u32,
      options: Vec<String>,
    }

    impl Draw for SelectBox {
      fn draw(&self) {
        println!("Draw SelectBox");
      }
    }

    // With traits, the system can be extended and another object can be added (another object that can draw). Methods are linked at runtime.
    let screen = Screen {
      components: vec![
        Box::new(SelectBox {
          width: 75,
          height: 10,
          options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
          ],
        }),
        Box::new(Button {
          width: 50,
          height: 10,
          label: String::from("OK"),
        }),
      ],
    };

    screen.run();

    // This one looks better with generic but it can not be extended. Methods are generated at compilation.
    let screen2 = Screen2 {
      components: vec![
        Button {
          width: 50,
          height: 10,
          label: String::from("OK"),
        },
      ],
    };

    screen2.run();

    // It might be to make it more open to other developers so they can add their own implementations.
  }
}
