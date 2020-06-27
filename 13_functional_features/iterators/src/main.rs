fn main() {
  // Traditional iterator, it unwraps the value automatically
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  for val in v1_iter {
    println!("v1_iter: {}", val);
  }

  // Here we see the actual value returned by next which is an Option
  let v1 = vec![1, 2, 3];
  let mut v1_iter = v1.iter();
  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);

  /*
    iter : returns immutable references to the value
    into_iter : takes ownership of the values
    iter_mut : iterates over mutable references of the values
  */

  // Iterators have functions to make common computations. Most of them use the iterator though.
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  let total: i32 = v1_iter.sum();
  assert_eq!(total, 6);

  // We can create another iterator from an iterator
  let v1: Vec<i32> = vec![1, 2, 3];
  let v2 = v1.iter().map(|x| x + 1);
  for val in v2 {
    println!("v2: {}", val);
  }

  // The code above is lazy, it gets executed only when we need it. To force Rust going though it, we can use collect.
  let v1: Vec<i32> = vec![1, 2, 3];
  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
  assert_eq!(v2, vec![2, 3, 4]);

  // We can filter values
  let shoes = vec![
    Shoe {
      size: 10,
      style: String::from("sneaker"),
    },
    Shoe {
      size: 13,
      style: String::from("sandal"),
    },
    Shoe {
      size: 10,
      style: String::from("boot"),
    },
  ];

  let in_my_size = shoes_in_my_size(shoes, 10);
  for val in in_my_size {
    println!("in_my_size: {:?}", val);
  }

  // We can also make our own iterators
  let c = Counter::new();
  for val in c {
    println!("c: {}", val);
  }
}

#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn filters_by_size() {
    let shoes = vec![
      Shoe {
        size: 10,
        style: String::from("sneaker"),
      },
      Shoe {
        size: 13,
        style: String::from("sandal"),
      },
      Shoe {
        size: 10,
        style: String::from("boot"),
      },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
      in_my_size,
      vec![
        Shoe {
          size: 10,
          style: String::from("sneaker")
        },
        Shoe {
          size: 10,
          style: String::from("boot")
        },
      ]
    );
  }

  // A fun one
  #[test]
  fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
      .zip(Counter::new().skip(1))
      .map(|(a, b)| a * b)
      .filter(|x| x % 3 == 0)
      .sum();
    assert_eq!(18, sum);
  }
}
