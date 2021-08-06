#![allow(dead_code, unused_imports, unused_variables)]

use image::DynamicImage;
use std::str::FromStr;

pub fn check_for_invalid_args(args: &Vec<String>, subcommand: &str, expected_length: usize) {
  if args.len() != expected_length {
    println!(
      "\n\x1b[31m[ERROR]: Invalid \x1b[1m{0}\x1b[0m\x1b[31m arguments. Expected \x1b[1m{1}\x1b[0m\x1b[31m argument(s), but received \x1b[1m{2}\x1b[0m\x1b[31m. See \x1b[1m{0}\x1b[0m\x1b[31m subcommand help for more assitance:\x1b[0m\n",
      subcommand,
      expected_length,
      args.len()
    );
    print_commands();
  };
}

pub fn print_commands() {
  println!("<subcommand> arguments\n");
  println!("blur         <amount(f32)> <input(String)> <output(String)>");
  println!("brighten     <amount(i32)> <input(String)> <output(String)>");
  println!(
    "crop         <x(u32)> <y(32)> <width(u32)> <height(u32)> <input(String)> <output(String)>"
  );
  println!("fractal      <output(String)>");
  println!("grayscale    <input(String)> <output(String)>");
  println!("invert       <input(String)> <output(String)>");
  println!("");
}

pub fn print_usage_and_exit() {
  println!("\nImage Manipulator");
  println!("Matt Carlotta <matt@mattcarlotta.sh>");
  println!("Manipulates images using the CLI\n");
  print_commands();
  std::process::exit(1);
}

pub fn open_image(input: String) -> DynamicImage {
  image::open(input).expect("Failed to open input.")
}

pub fn save_image(img: DynamicImage, output: String) {
  img.save(output).expect("Failed writing output.");
}

pub fn parse_number<T>(subcommand: &String, property: String, num_str: String) -> T
where
  T: FromStr,
  <T as std::str::FromStr>::Err: std::fmt::Debug,
{
  let amount = num_str.parse::<T>().unwrap_or_else(|_error| {
    println!(
      "\n\x1b[31m[ERROR]: The \x1b[1m{}\x1b[0m\x1b[31m argument contains an invalid number. See \x1b[1m{}\x1b[0m\x1b[31m subcommand help for more assitance.\x1b[0m\n", property, subcommand
    );

    print_commands();

    std::process::exit(1);
  });

  return amount;
}

pub fn blur(amount: f32, input: String, output: String) {
  // Here's how you open an existing image file
  let img = open_image(input);
  // **OPTION**
  // Parse the blur amount (an f32) from the command-line and pass it through
  // to this function, instead of hard-coding it to 2.0.
  let new_image = img.blur(amount);
  // Here's how you save an image to a file.
  save_image(new_image, output);
}

pub fn brighten(amount: i32, input: String, output: String) {
  // See blur() for an example of how to open / save an image.
  let img = open_image(input);
  // .brighten() takes one argument, an i32.  Positive numbers brighten the
  // image. Negative numbers darken it.  It returns a new image.
  let new_image = img.brighten(amount);
  // Challenge: parse the brightness amount from the command-line and pass it
  // through to this function.

  save_image(new_image, output);
}

pub fn crop(options: Vec<u32>, input: String, output: String) {
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

pub fn rotate(input: String, output: String) {
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

pub fn invert(input: String, output: String) {
  let mut img = open_image(input);

  // .invert() takes no arguments and converts the image in-place, so you
  // will use the same image to save out to a different file.
  img.invert();

  save_image(img, output);
}

pub fn grayscale(input: String, output: String) {
  let img = open_image(input);

  // .grayscale() takes no arguments. It returns a new image.
  let new_image = img.grayscale();

  save_image(new_image, output);
}

pub fn generate(output: String) {
  // Create an ImageBuffer -- see fractal() for an example

  // Iterate over the coordinates and pixels of the image -- see fractal() for an example

  // Set the image to some solid color. -- see fractal() for an example

  // Challenge: parse some color data from the command-line, pass it through
  // to this function to use for the solid color.

  // Challenge 2: Generate something more interesting!

  // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
pub fn fractal(output: String) {
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
