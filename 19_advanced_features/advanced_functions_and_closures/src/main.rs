fn main() {
  // We can use a function previously defined as a function pointer
  {
    fn add_one(x: i32) -> i32 {
      x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
      f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // Here is a lambda just to remind me how it works
    let answer = do_twice(|x| x + 2, 5);
    println!("The answer is: {}", answer);

    // Another one
    let lambda = |x| x + 2;
    let answer = do_twice(lambda, 5);
    println!("The answer is: {}", answer);
  }

  // Examples with map
  {
    let list_of_numbers = vec![1, 2, 3];
    #[allow(unused_variables)]
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_numbers = vec![1, 2, 3];
    #[allow(unused_variables)]
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Status {
      Value(u32),
      Stop,
    }

    // This generates a list of u32 from 0 to 20 and shove each value in a Status
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("list_of_statuses {:?}", list_of_statuses);
  }

  // To return a closure, it looks like we need Box
  {
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
      Box::new(|x| x + 1)
    }
    println!("{}", returns_closure()(1));
  }
}
