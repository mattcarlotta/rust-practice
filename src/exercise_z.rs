#![allow(dead_code, unused_imports, unused_variables)]

use image::DynamicImage;
use std::str::FromStr;

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
    // EXAMPLE FOR CONVERSION OPEamountNS
    "blur" => {
      let (amount, input, output): (f32, String, String) =
        parse_common_args::<f32>(&subcommand, &mut args);
      blur(amount, input, output);
    }
    // **OPTION**
    // Brighten -- see the brighten() function below
    "brighten" => {
      let (amount, input, output): (i32, String, String) =
        parse_common_args::<i32>(&subcommand, &mut args);
      // let primitive_amount =
      // **OPTION**
      // Improve the blur implementation -- see the blur() function below
      brighten(amount, input, output);
    }
    // **OPTION**
    // Crop -- see the crop() function below
    "crop" => {
      if args.len() != 6 {
        print_usage_and_exit();
      };

      let mut options: [u32; 4] = [0; 4];
      let properties: [&str; 4] = ["x", "y", "width", "height"];
      for x in 0..options.len() {
        options[x] = parse_number::<u32>(&subcommand, properties[x].to_string(), args.remove(0));
      }
      // let primitive_amount =
      // **OPTION**
      // Improve the blur implementation -- see the blur() function below
      crop(options, args.remove(0), args.remove(0));
    }
    // **OPTION**
    // Grayscale -- see the grayscale() function below
    "grayscale" => {
      if args.len() != 2 {
        print_usage_and_exit();
      };

      grayscale(args.remove(0), args.remove(0));
    }

    // **OPTION**
    // Rotate -- see the rotate() function below

    // **OPTION**
    // Invert -- see the invert() function below
    "invert" => {
      if args.len() != 2 {
        print_usage_and_exit();
      };

      invert(args.remove(0), args.remove(0));
    }

    // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
    "fractal" => {
      if args.len() != 1 {
        print_usage_and_exit();
      }
      let output = args.remove(0);
      fractal(output);
    }
    "help" => {
      print_usage_and_exit();
    }

    // **OPTION**
    // Generate -- see the generate() function below -- this should be sort of like "fractal()"!

    // For everything else...
    _ => {
      print_usage_and_exit();
    }
  }
}

fn open_image(input: String) -> DynamicImage {
  image::open(input).expect("Failed to open input.")
}

fn save_image(img: DynamicImage, output: String) {
  img.save(output).expect("Failed writing output.");
}

fn print_usage_and_exit() {
  println!("\nImage Manipulator");
  println!("Matt Carlotta <matt@mattcarlotta.sh>");
  println!("Manipulates images using the CLI\n");
  println!("<subcommand> arguments\n");
  println!("blur         <amount(f32)> <input(String)> <output(String)>");
  println!("brighten     <amount(i32)> <input(String)> <output(String)>");
  println!(
    "crop         <x(u32)> <y(32)> <width(u32)> <height(u32)> <input(String)> <output(String)>"
  );
  println!("fractal      <output(String)>");
  println!("grayscale    <input(String)> <output(String)>");
  println!("invert       <input(String)> <output(String)>");

  std::process::exit(-1);
}

fn blur(amount: f32, input: String, output: String) {
  // Here's how you open an existing image file
  let img = open_image(input);
  // **OPTION**
  // Parse the blur amount (an f32) from the command-line and pass it through
  // to this function, instead of hard-coding it to 2.0.
  let new_image = img.blur(amount);
  // Here's how you save an image to a file.
  save_image(new_image, output);
}

fn brighten(amount: i32, input: String, output: String) {
  // See blur() for an example of how to open / save an image.
  let img = open_image(input);
  // .brighten() takes one argument, an i32.  Positive numbers brighten the
  // image. Negative numbers darken it.  It returns a new image.
  let new_image = img.brighten(amount);
  // Challenge: parse the brightness amount from the command-line and pass it
  // through to this function.

  save_image(new_image, output);
}

fn crop(options: [u32; 4], input: String, output: String) {
  // See blur() for an example of how to open an image.
  let mut img = open_image(input);
  // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
  // You may hard-code them, if you like.  It returns a new image.
  let new_image = img.crop(options[0], options[1], options[2], options[3]);
  // Challenge: parse the four values from the command-line and pass them
  // through to this function.

  save_image(new_image, output);
  // See blur() for an example of how to save the image.
}

fn rotate(input: String, output: String) {
  // See blur() for an example of how to open an image.

  // There are 3 rotate functions to choose from (all clockwise):
  //   .rotate90()
  //   .rotate180()
  //   .rotate270()
  // All three methods return a new image.  Pick one and use it!

  // Challenge: parse the rotation amount from the command-line, pass it
  // through to this function to select which method to call.

  // See blur() for an example of how to save the image.
}

fn invert(input: String, output: String) {
  let mut img = open_image(input);

  // .invert() takes no arguments and converts the image in-place, so you
  // will use the same image to save out to a different file.
  img.invert();

  save_image(img, output);
}

fn grayscale(input: String, output: String) {
  let img = open_image(input);

  // .grayscale() takes no arguments. It returns a new image.
  let new_image = img.grayscale();

  save_image(new_image, output);
}

fn generate(output: String) {
  // Create an ImageBuffer -- see fractal() for an example

  // Iterate over the coordinates and pixels of the image -- see fractal() for an example

  // Set the image to some solid color. -- see fractal() for an example

  // Challenge: parse some color data from the command-line, pass it through
  // to this function to use for the solid color.

  // Challenge 2: Generate something more interesting!

  // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(output: String) {
  let width = 800;
  let height = 800;

  let mut imgbuf = image::ImageBuffer::new(width, height);

  let scale_x = 3.0 / width as f32;
  let scale_y = 3.0 / height as f32;

  // Iterate over the coordinates and pixels of the image
  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    // Use red and blue to be a pretty gradient background
    let red = (0.3 * x as f32) as u8;
    let blue = (0.3 * y as f32) as u8;

    // Use green as the fractal foreground (here is the fractal math part)
    let cx = y as f32 * scale_x - 1.5;
    let cy = x as f32 * scale_y - 1.5;

    let c = num_complex::Complex::new(-0.4, 0.6);
    let mut z = num_complex::Complex::new(cx, cy);

    let mut green = 0;
    while green < 255 && z.norm() <= 2.0 {
      z = z * z + c;
      green += 1;
    }

    // Actually set the pixel. red, green, and blue are u8 values!
    *pixel = image::Rgb([red, green, blue]);
  }

  imgbuf.save(output).unwrap();
}

fn parse_number<T>(subcommand: &String, property: String, num_str: String) -> T
where
  T: FromStr,
  <T as std::str::FromStr>::Err: std::fmt::Debug,
{
  let amount = num_str.parse::<T>().unwrap_or_else(|_error| {
    println!(
      "\n\x1b[31m[ERROR]: The \x1b[1m{}\x1b[0m\x1b[31m argument contains an invalid number. See \x1b[1m{}\x1b[0m\x1b[31m help for more assitance.\x1b[0m\n", property, subcommand
    );

    std::process::exit(1);
  });

  return amount;
}

fn parse_common_args<T>(subcommand: &String, args: &mut Vec<String>) -> (T, String, String)
where
  T: FromStr,
  <T as std::str::FromStr>::Err: std::fmt::Debug,
{
  if args.len() != 3 {
    print_usage_and_exit();
  };

  let amount = parse_number::<T>(&subcommand, "amount".to_string(), args.remove(0));
  let input = args.remove(0);
  let output = args.remove(0);

  return (amount, input, output);
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
