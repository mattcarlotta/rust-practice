use image_utils::*;
use std::env;

// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

pub fn run() {
  // 1. First, you need to implement some basic command-line argument handling
  // so you can make your program do different things.  Here's a little bit
  // to get you started doing manual parsing.
  //
  // Challenge: If you're feeling really ambitious, you could delete this code
  // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
  let mut args: Vec<String> = env::args().skip(1).collect();
  if args.is_empty() {
    print_usage_and_exit();
  }
  match args.remove(0).as_str() {
    "blur" => blur(&mut args),
    "brighten" => brighten(&mut args),
    "crop" => crop(&mut args),
    "fractal" => fractal(&mut args),
    // **Generate** -- see the generate() function below -- this should be sort of like "fractal()"!
    "grayscale" => grayscale(&mut args),
    "help" => print_usage_and_exit(),
    "invert" => invert(&mut args),
    // **Rotate** -- see the rotate() function below
    _ => print_usage_and_exit(),
  }
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run input.png output.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read input.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to output.png
//
// Good luck!
