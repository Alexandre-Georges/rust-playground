fn main() {
  /*
    A macro is code that writes code, saving us some time.
    println! and vec! are macros that generate code, they do not have a pre-determined number of parameters.
  */

  // Here is the vector macro which is a declarative macro
  {
    // This brings the macro into the scope when the create is imported.
    #[macro_export]
    macro_rules! vec {
      // ( $( $x:expr ),* ) is a pattern matcher that executes the macro if it matches
      ( $( $x:expr ),* ) => {
        {
          let mut temp_vec = Vec::new();
          // This is generated for each part that matches the pattern
          $(
            temp_vec.push($x);
          )*
          temp_vec
        }
      };
    }
  }

  // More macros in the other project
}
