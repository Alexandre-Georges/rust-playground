use std::ops::Deref;

fn main() {
  /*
   Box is used to store data in the heap instead of the stack.
   It is useful when :
   - we don't know the size of the data at compile time
   - we have a lot of data and don't want to clone it
   - we want to own a value and don't care about its type, we just need to know that the object implements some traits
  */

  // We can create a new box like that :
  let b = Box::new(5);
  println!("b = {}", b);

  {

    enum List {
      Cons(i32, Box<List>),
      Nil,
    }

    use List::{Cons, Nil};

    // The box goes in the stack and the value in the heap.
    // The memory is freed when the pointer goes outside the scope.

    // Box is useful when we have type recursion : a type A that contains somewhere in it another A
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  }

  // Box implements Deref meaning it is a reference and Drop for cleaning up when it leaves the scope.

  // Box behaves like a regular pointer
  let x = 5;
  let y = &x;
  let z = Box::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
  assert_eq!(5, *z);

  // Let's implement our own version
  struct MyBox<T>(T);

  impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
      MyBox(x)
    }
  }

  // We now need to provide Rust with a method to de-reference the structure
  impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
      // This somehow provides a reference to the target
      &self.0
    }
  }
  let a = MyBox::new(x);
  assert_eq!(5, *a);

  /*
    Using &m uses deref to extract the string that mybox contains.
    It turns &MyBox<String> into &String and Rust calls deref again, on String this time, to get &str.
  */
  fn hello(name: &str) {
    println!("Hello, {}!", name);
  }
  let m = MyBox::new(String::from("Rust"));
  hello(&m);

  /*
    The trait DerefMut can be used to override * on mutable references.
    Rust can go from : immutable to immutable, mutable to mutable or mutable to immutable.
    It can't be immutable to mutable though.
  */

  // When a variable gets out of scope, Rust calls the Drop trait to clean after it
  struct CustomSmartPointer {
    data: String,
  }

  impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
      println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
  }

  fn test_drop() {
    let c = CustomSmartPointer {
      data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
  }
  test_drop();

  {
    let c = CustomSmartPointer {
      data: String::from("my stuff"),
    };
    // We can also make Rust drop a value early
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
  }


  /*
    In some cases a variable can have multiple owners, we use Rc for that.
    This is a reference counting pointer, when the counter is at 0 the variable
    does not have any owners left and it can be dropped.
  */
  {
    enum List {
      Cons(i32, Rc<List>),
      Nil,
    }

    use std::rc::Rc;
    use List::{Cons, Nil};

    fn test_rc() {
      // Here 2 lists reuse another one
      let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
      println!("count after creating a = {}", Rc::strong_count(&a));

      // This is not a deep clone
      let b = Cons(3, Rc::clone(&a));
      println!("count after creating b = {}", Rc::strong_count(&a));
      {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
      }
      println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
    test_rc();
  }

  /*
    RefCell<T> is similar to Box<T>, the difference is that Box checks that :
    - we have at most one mutable reference or unlimited immutable references
    - references are always valid
    at compile time. RefCell does it at runtime so it will compile.
    Apparently it would be useful to mutate local variables which are exposed as immutable values to another piece of code.

    Recap :
    - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
    - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
  */

}
// Here is an example when mocking objects that requires RefCell

pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger,
{
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 1.0 {
      self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max >= 0.9 {
      self.messenger
        .send("Urgent warning: You've used up over 90% of your quota!");
    } else if percentage_of_max >= 0.75 {
      self.messenger
        .send("Warning: You've used up over 75% of your quota!");
    }
  }
}

/*
  This code does not compile because
  ```
  fn send(&self, message: &str) {
    self.sent_messages.push(String::from(message));
  }
  ```
  tries to modify self which is immutable.

#[cfg(test)]
mod tests {
  use super::*;

  struct MockMessenger {
    sent_messages: Vec<String>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        sent_messages: vec![],
      }
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      self.sent_messages.push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messages.len(), 1);
  }
}
*/
#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      // We make the variable mutable to keep track of the messages
      self.sent_messages.borrow_mut().push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }
}

/*
  borrow returns a Ref<T>, borrow_mut returns a RefMut<T>
  RefCell keeps track of the references for each borrow and borrow_mut
  The compiler will let us borrow 2 mutable references but it will panick at execution.
*/

// If we use Rc and RefCell we can have multiple owners and mutate values.
pub fn test_multiple_owners_mutable() {
  #[derive(Debug)]
  enum List {
      Cons(Rc<RefCell<i32>>, Rc<List>),
      Nil,
  }

  use List::{Cons, Nil};
  use std::cell::RefCell;
  use std::rc::Rc;

  let value = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

  let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

  *value.borrow_mut() += 10;

  println!("a after = {:?}", a);
  println!("b after = {:?}", b);
  println!("c after = {:?}", c);
}
