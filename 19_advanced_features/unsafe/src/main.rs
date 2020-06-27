fn main() {
  /*
    We can use unsafe Rust to :
    - dereference a raw pointer
    - call an unsafe function or method
    - access or modify a mutable static variable
    - implement an unsafe trait
    - access fields of unions
  */
  {
    // Raw pointers
    let mut num = 5;

    // Those are raw pointers, an immutable one and a mutable one
    #[allow(unused_variables)]
    let r1 = &num as *const i32;
    #[allow(unused_variables)]
    let r2 = &mut num as *mut i32;

    // It is unsafe to dereference a pointer to read its value
    unsafe {
      println!("r1 is: {}", *r1);
      println!("r2 is: {}", *r2);
    }
  }

  {
    // Calling an unsafe function requires an unsafe block
    unsafe fn dangerous() {}
    unsafe {
      dangerous();
    }

    // We can make unsafe code safe
    use std::slice;

    #[allow(dead_code)]
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
      let len = slice.len();
      let ptr = slice.as_mut_ptr();

      assert!(mid <= len);

      unsafe {
        (
          slice::from_raw_parts_mut(ptr, mid),
          slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
      }
    }
  }

  {
    // Rust can call external code, like C
    extern "C" {
      fn abs(input: i32) -> i32;
    }
    unsafe {
      println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // We can also make Rust code callable from another language. No mangle means that the name of the function is preserved.
    #[no_mangle]
    pub extern "C" fn call_from_c() {
      println!("Just called a Rust function from C!");
    }
  }

  // Mutating static variables is also unsafe
  {
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
      unsafe {
        COUNTER += inc;
      }
    }
    add_to_count(3);

    unsafe {
      println!("COUNTER: {}", COUNTER);
    }
  }

  // Unsafe trait
  {
    unsafe trait Foo {
      // methods go here
    }

    unsafe impl Foo for i32 {
      // method implementations go here
    }
  }
}
