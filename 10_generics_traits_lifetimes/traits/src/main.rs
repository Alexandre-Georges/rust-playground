fn main() {
  pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
  }

  // A trait is similar to an interface
  pub trait Summary {
    fn summarize(&self) -> String;
  }

  // We can then implement the trait
  impl Summary for Tweet {
    fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
    }
  }

  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };

  println!("summarize: {}", tweet.summarize());

  /*
    Traits can only be implemented where the class that implements them is defined
  */

  // We can also define a default behaviour
  pub trait Summary2 {
    fn summarize2(&self) -> String {
      String::from("(Read more...)")
    }
  }
  impl Summary2 for Tweet {}


  println!("summarize2: {}", tweet.summarize2());

  // We can use traits similar to abstract classes
  pub trait Summary3 {
    fn summarize_author3(&self) -> String;

    fn summarize3(&self) -> String {
      format!("(Read more from {}...)", self.summarize_author3())
    }
  }
  // Now we just need to define the abstract method
  impl Summary3 for Tweet {
    fn summarize_author3(&self) -> String {
      format!("@{}", self.username)
    }
  }
  println!("summarize3: {}", tweet.summarize3());


  // We can define functions with traits as parameter
  pub fn notify(item: &impl Summary) {
    println!("notify {}", item.summarize());
  }
  notify(&tweet);

  // The function above is syntax sugar for
  pub fn notify2<T: Summary>(item: &T) {
    println!("notify2 {}", item.summarize());
  }
  notify2(&tweet);


  // However there is a difference: item1 and item2 can have different types
  pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("notify3 {} {}", item1.summarize(), item2.summarize());
  }
  notify3(&tweet, &tweet);

  // Here they must have the same type
  pub fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!("notify4 {} {}", item1.summarize(), item2.summarize());
  }
  notify4(&tweet, &tweet);


  pub trait Display2 {
    fn display(&self) -> String { String::from("Display") }
  }
  impl Display2 for Tweet {}
  // A parameter can have multiple traits
  pub fn notify5(item: &(impl Summary + Display2)) {
    println!("notify5 {} {}", item.summarize(), item.display());
  }
  notify5(&tweet);

  // Or like that
  pub fn notify6<T: Summary + Display2>(item: &T) {
    println!("notify6 {} {}", item.summarize(), item.display());
  }
  notify6(&tweet);


  // We can end up with big signatures
  fn some_function<T: Display2 + Clone, U: Clone + Summary>(t: T, u: U) -> i32 {
    println!("some_function {} {}", t.display(), u.summarize());
    return 1;
  }

  // We can use this alternative syntax to make it easier to read
  fn some_function2<T, U>(t: T, u: U) -> i32
    where T: Display2 + Clone,
          U: Clone + Summary {
    println!("some_function2 {} {}", t.display(), u.summarize());
    return 1;
  }


  // We can return an object that implements a trait like so
  fn returns_summarizable() -> impl Summary {
    Tweet {
      username: String::from("horse_ebooks"),
      content: String::from("of course, as you probably already know, people"),
      reply: false,
      retweet: false,
    }
  }
  // To circle back to the initial issue, Copy is necessary as we "copy" the element
  fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
      if item > largest {
        largest = item;
      }
    }

    largest
  }
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {}", result);


  // Another example
  use std::fmt::Display;

  struct Pair<T> {
    x: T,
    y: T,
  }

  impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
      Self { x, y }
    }
  }

  impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
      if self.x >= self.y {
        println!("The largest member is x = {}", self.x);
      } else {
        println!("The largest member is y = {}", self.y);
      }
    }
  }
  let p = Pair::new(1, 2);
  p.cmp_display();


  // We can add a new behaviour to a trait
  impl<T: Display> Display2 for T {
  }
}
