#![allow(dead_code, unused_imports, unused_variables)]

use image::{DynamicImage, ImageError, ImageResult};
use std::io::ErrorKind;
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
    // EXAMPLE FOR CONVERSION OPERATIONS
    "blur" => {
      let (ratio, infile, outfile): (f32, String, String) =
        parse_common_args::<f32>(subcommand, &mut args);
      blur(ratio, infile, outfile);
    }
    "brighten" => {
      let (ratio, infile, outfile): (i32, String, String) =
        parse_common_args::<i32>(subcommand, &mut args);
      // let primitive_ratio =
      // **OPTION**
      // Improve the blur implementation -- see the blur() function below
      brighten(ratio, infile, outfile);
    }

    // **OPTION**
    // Brighten -- see the brighten() function below

    // **OPTION**
    // Crop -- see the crop() function below

    // **OPTION**
    // Rotate -- see the rotate() function below

    // **OPTION**
    // Invert -- see the invert() function below

    // **OPTION**
    // Grayscale -- see the grayscale() function below

    // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
    "fractal" => {
      if args.len() != 1 {
        print_usage_and_exit();
      }
      let outfile = args.remove(0);
      fractal(outfile);
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

fn open_image(infile: String) -> DynamicImage {
  let img = image::open(infile).expect("Failed to open INFILE.");

  return img;
}

fn print_usage_and_exit() {
  println!("\n BASIC USAGE");
  println!("-----------------------------------------------------------");
  println!("| command   | arguments                                   |");
  println!("-----------------------------------------------------------");
  println!("| blur      | RATIO(f32) INFILE(String) OUTFILE(String)   |");
  println!("| brighten  | RATIO(i32) INFILE(String) OUTFILE(String)   |");
  println!("-----------------------------------------------------------");

  std::process::exit(-1);
}

fn blur(ratio: f32, infile: String, outfile: String) {
  // Here's how you open an existing image file
  let img = open_image(infile);
  // **OPTION**
  // Parse the blur amount (an f32) from the command-line and pass it through
  // to this function, instead of hard-coding it to 2.0.
  let new_image = img.blur(ratio);
  // Here's how you save an image to a file.
  new_image.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(ratio: i32, infile: String, outfile: String) {
  // See blur() for an example of how to open / save an image.
  let img = open_image(infile);
  // .brighten() takes one argument, an i32.  Positive numbers brighten the
  // image. Negative numbers darken it.  It returns a new image.
  let new_image = img.brighten(ratio);
  // Challenge: parse the brightness amount from the command-line and pass it
  // through to this function.

  new_image.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String) {
  // See blur() for an example of how to open an image.

  // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
  // You may hard-code them, if you like.  It returns a new image.

  // Challenge: parse the four values from the command-line and pass them
  // through to this function.

  // See blur() for an example of how to save the image.
}

fn rotate(infile: String, outfile: String) {
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

fn invert(infile: String, outfile: String) {
  // See blur() for an example of how to open an image.

  // .invert() takes no arguments and converts the image in-place, so you
  // will use the same image to save out to a different file.

  // See blur() for an example of how to save the image.
}

fn grayscale(infile: String, outfile: String) {
  // See blur() for an example of how to open an image.

  // .grayscale() takes no arguments. It returns a new image.

  // See blur() for an example of how to save the image.
}

fn generate(outfile: String) {
  // Create an ImageBuffer -- see fractal() for an example

  // Iterate over the coordinates and pixels of the image -- see fractal() for an example

  // Set the image to some solid color. -- see fractal() for an example

  // Challenge: parse some color data from the command-line, pass it through
  // to this function to use for the solid color.

  // Challenge 2: Generate something more interesting!

  // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
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

  imgbuf.save(outfile).unwrap();
}

fn parse_common_args<T>(subcommand: String, args: &mut Vec<String>) -> (T, String, String)
where
  T: FromStr,
  <T as std::str::FromStr>::Err: std::fmt::Debug,
{
  if args.len() != 3 {
    print_usage_and_exit();
  }
  let ratio = args.remove(0).parse::<T>().unwrap_or_else(|_error| {
    println!(
      "\n\x1b[31m[ERROR]: The \"ratio\" argument contains an invalid number. See \x1b[1m\"{}\"\x1b[0m\x1b[31m help for more assitance.\x1b[0m\n", subcommand
    );

    std::process::exit(1);
  });
  let infile = args.remove(0);
  let outfile = args.remove(0);

  return (ratio, infile, outfile);
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
