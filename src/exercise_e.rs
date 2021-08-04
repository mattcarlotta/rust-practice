// Silence some warnings so they don't distract from the exercise.
// #![allow(unused_mut)]

pub fn run() {
  // This fancy stuff either gets the first argument as a String, or prints
  // usage and exits if an argument was not supplied to the program.
  let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
    println!("Excercise E: Please supply an argument to this program.");
    std::process::exit(-1);
  });

  // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
  // prints whether the contents of the String is plural or singular. Then uncomment and run this
  // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
  // String reference
  //
  inspect(&arg);

  // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
  // the String if it doesn't already end with "s". Then uncomment and run the code below with
  // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
  //
  change(&mut arg);
  //println!("I have many {}", arg);

  // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
  // indicating whether or not the String both starts with a "b" AND contains an "a".
  // Hint 1: use `.starts_with("b")` and `.contains("a")`
  // Hint 2: `&&` is the boolean "AND" operator
  //
  //if eat(arg) {
  //    println!("Might be bananas");
  //} else {
  //    println!("Not bananas");
  //}
  eat(arg);

  // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

  // Challenge: Write a function "add" that takes *references* to two integer arguments,
  // dereferences them and adds them together, and returns the result.
  //
  // println!("1 + 2 = {}, even via references", add(&1, &2));
  add(&1, &2);
}

// 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
// prints whether the contents of the String is plural or singular. Then uncomment and run this
// code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
// String reference
//
fn inspect(s: &String) {
  let case = if s.ends_with("s") {
    "plural"
  } else {
    "singular "
  };

  println!("Exercise E: {} is {}", s, case);
}

// 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
// the String if it doesn't already end with "s". Then uncomment and run the code below with
// `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
//
fn change(s: &mut String) {
  let arg = s.clone();
  let mut message = String::from("was not changed");

  if !s.ends_with("s") {
    s.push_str("s");
    message = message.replace("not ", "");
  };

  println!("Exercise E: Argument {} {} to {}", arg, message, s);
}

// 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
// indicating whether or not the String both starts with a "b" AND contains an "a".
// Hint 1: use `.starts_with("b")` and `.contains("a")`
// Hint 2: `&&` is the boolean "AND" operator
//
fn eat(arg: String) -> bool {
  let is_banana = arg.starts_with("b") && arg.contains("a");

  let message = if is_banana {
    "Might be bananas"
  } else {
    "Not bananas"
  };

  println!("Exercise E: {}", message);

  return is_banana;
}

// Challenge: Write a function "add" that takes *references* to two integer arguments,
// dereferences them and adds them together, and returns the result.
//
// println!("1 + 2 = {}, even via references", add(&1, &2));
fn add(a: &i32, b: &i32) -> i32 {
  let sum = *a + *b;

  println!("Exercise E: {} + {} = {}, even via references", a, b, sum);

  return sum;
}
