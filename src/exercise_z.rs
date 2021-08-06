use image_utils::*;

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
  let mut args: Vec<String> = std::env::args().skip(1).collect();
  if args.is_empty() {
    print_usage_and_exit();
  }
  let subcommand = args.remove(0);
  match subcommand.as_str() {
    "blur" => {
      check_for_invalid_args(&args, &subcommand, 3);

      let amount = parse_number::<f32>(&subcommand, "amount".to_string(), args.remove(0));

      blur(amount, args.remove(0), args.remove(0));
    }
    "brighten" => {
      check_for_invalid_args(&args, &subcommand, 3);

      let amount = parse_number::<i32>(&subcommand, "amount".to_string(), args.remove(0));

      brighten(amount, args.remove(0), args.remove(0));
    }
    "crop" => {
      check_for_invalid_args(&args, &subcommand, 6);

      let mut options = Vec::with_capacity(4);
      for property in ["x", "y", "width", "height"] {
        options.push(parse_number::<u32>(
          &subcommand,
          property.to_string(),
          args.remove(0),
        ));
      }

      crop(options, args.remove(0), args.remove(0));
    }
    "grayscale" => {
      check_for_invalid_args(&args, &subcommand, 2);

      grayscale(args.remove(0), args.remove(0));
    }
    // **Rotate** -- see the rotate() function below
    "invert" => {
      check_for_invalid_args(&args, &subcommand, 2);

      invert(args.remove(0), args.remove(0));
    }
    "fractal" => {
      check_for_invalid_args(&args, &subcommand, 1);

      let output = args.remove(0);
      fractal(output);
    }
    "help" => {
      print_usage_and_exit();
    }
    // **Generate** -- see the generate() function below -- this should be sort of like "fractal()"!
    _ => {
      print_usage_and_exit();
    }
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
