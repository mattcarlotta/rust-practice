use image::{DynamicImage, ImageBuffer};
use num_complex::Complex;
use std::process::exit;
use std::str::FromStr;

/// Checks if a command was passed the correct number of arguments.
///
/// Arguments:
///
/// * `args` - &Vec<String>
/// * `subcommand` - &str
/// * `expected_length` - usize
///
/// Usage: ```check_for_invalid_args(args, "crop", 6);```
pub fn check_for_invalid_args(
  args: &Vec<String>,
  subcommand: &str,
  expected_length: usize,
) -> Option<()> {
  if args.is_empty() || args.len() != expected_length {
    println!(
      "\n\x1b[31m[ERROR]: Invalid \x1b[1m{0}\x1b[0m\x1b[31m arguments. Expected \x1b[1m{1}\x1b[0m\x1b[31m argument(s), but received \x1b[1m{2}\x1b[0m\x1b[31m. See the \x1b[1m{0}\x1b[0m\x1b[31m subcommand help for more assitance:\x1b[0m\n",
      subcommand,
      expected_length,
      args.len()
    );
    print_commands();
    return None;
  };

  return Some(());
}

/// Prints commands and their arguments.
pub fn print_commands() {
  println!("subcommand   <arguments>\n");
  println!("blur         <amount(f32)> <input(String)> <output(String)>");
  println!("brighten     <amount(i32)> <input(String)> <output(String)>");
  println!(
    "crop         <x(u32)> <y(u32)> <width(u32)> <height(u32)> <input(String)> <output(String)>"
  );
  println!("fractal      <output(String)>");
  println!(
    "generate     <width(u32)> <height(u32)> <red(u8)> <green(u8)> <blue(u8)> <output(String)>"
  );
  println!("grayscale    <input(String)> <output(String)>");
  println!("invert       <input(String)> <output(String)>");
  println!("rotate       <amount(i32 -> 90 | 180 | 270)> <input(String)> <output(String)>");
  println!("");
}

/// Prints application usage information.
pub fn print_usage_and_exit() {
  println!("\nImage Manipulator");
  println!("Matt Carlotta <matt@mattcarlotta.sh>");
  println!("Manipulates images using the CLI\n");
  print_commands();
  exit(1);
}

/// Open a file image.
///
/// Arguments:
///
/// * `input` - String
///
/// Usage: ```open_image("input.png");```
pub fn open_image(input: String) -> DynamicImage {
  image::open(input).expect("Failed to open input.")
}

/// Saves a dynamic image.
///
/// Arguments:
///
/// * `img` - DynamicImage
/// * `output` - String
///
/// Usage: ```save_image(image, "output.png");```
pub fn save_image(img: DynamicImage, output: String) {
  img.save(output).expect("Failed writing output.");

  exit(0);
}

/// Dynamically parses a string to a number type for a `command` and `property`.
///
/// Arguments:
///
/// * `subcommand` - String
/// * `property` - String
/// * `num_str` - String
///
/// Usage: ```parse_number("crop", "x", "255");```
pub fn parse_number<T: FromStr>(subcommand: &str, property: &str, num_str: String) -> T {
  num_str.parse::<T>().unwrap_or_else(|_error| {
    println!(
      "\n\x1b[31m[ERROR]: The \x1b[1m<{0}>\x1b[0m\x1b[31m argument passed to \x1b[1m{1}\x1b[0m\x1b[31m is an invalid number. See the \x1b[1m{1}\x1b[0m\x1b[31m subcommand help for more assitance:\x1b[0m\n", property, subcommand
    );

    print_commands();

    exit(1);
  })
}

/// Blurs an image by an `amount`.
///
/// Arguments:
///
/// * `amount` - u32
/// * `input` - String
/// * `output` - String
///
/// Usage: ```blur 100 input.png output.png```
pub fn blur(args: &mut Vec<String>) {
  match check_for_invalid_args(&args, "blur", 3) {
    Some(()) => {
      let amount = parse_number::<f32>("blur", "amount", args.remove(0));
      // Here's how you open an existing image file
      let img = open_image(args.remove(0));
      // **OPTION**
      // Parse the blur amount (an f32) from the command-line and pass it through
      // to this function, instead of hard-coding it to 2.0.
      let new_image = img.blur(amount);
      // Here's how you save an image to a file.
      save_image(new_image, args.remove(0));
    }
    None => exit(1),
  }
}

/// Brightens an image by an `amount`.
///
/// Arguments:
///
/// * `amount` - u32
/// * `input` - String
/// * `output` - String
///
/// Usage: ```brighten 100 input.png output.png```
pub fn brighten(args: &mut Vec<String>) {
  match check_for_invalid_args(&args, "brighten", 3) {
    Some(()) => {
      let amount = parse_number::<i32>("brighten", "amount", args.remove(0));
      // See blur() for an example of how to open / save an image.
      let img = open_image(args.remove(0));
      // .brighten() takes one argument, an i32.  Positive numbers brighten the
      // image. Negative numbers darken it.  It returns a new image.
      let new_image = img.brighten(amount);
      // Challenge: parse the brightness amount from the command-line and pass it
      // through to this function.
      save_image(new_image, args.remove(0));
    }
    None => exit(1),
  }
}

/// Crops an image by `x`, `y`, `width` and `height`.
///
/// Arguments:
///
/// * `x` - u32
/// * `y` - u32
/// * `width` - u32
/// * `height` - u32
/// * `input` - String
/// * `output` - String
///
/// Usage: ```crop 0 0 640 480 input.png output.png```
pub fn crop(args: &mut Vec<String>) {
  match check_for_invalid_args(&args, "crop", 6) {
    Some(()) => {
      let [x, y, width, height] =
        ["x", "y", "width", "height"].map(|opt| parse_number::<u32>("crop", opt, args.remove(0)));

      // See blur() for an example of how to open an image.
      let mut img = open_image(args.remove(0));
      // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
      // You may hard-code them, if you like.  It returns a new image.
      let new_image = img.crop(x, y, width, height);
      // Challenge: parse the four values from the command-line and pass them
      // through to this function.

      save_image(new_image, args.remove(0));
    }
    None => exit(1),
  }
}

/// Generates a fractal image.
///
/// Arguments:
///
/// * `output` - String
///
/// Usage: ```fractal output.png```
pub fn fractal(args: &mut Vec<String>) {
  match check_for_invalid_args(&args, "fractal", 1) {
    Some(()) => {
      let width = 800;
      let height = 800;

      let mut imgbuf = ImageBuffer::new(width, height);

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

        let c = Complex::new(-0.4, 0.6);
        let mut z = Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
          z = z * z + c;
          green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
      }

      imgbuf.save(args.remove(0)).unwrap();
    }
    None => exit(1),
  }
}

/// Generates an image by `width`, `height`, `red`, `green` and `blue`.
///
/// Arguments:
///
/// * `width` - u32
/// * `height` - u32
/// * `red` - u8
/// * `green` - u8
/// * `blue` - u8
/// * `output` - String
///
/// Usage: ```generate 640 480 0 0 0 input.png output.png```
pub fn generate(args: &mut Vec<String>) {
  match check_for_invalid_args(&args, "rotate", 6) {
    Some(()) => {
      let [width, height] =
        ["width", "height"].map(|d| parse_number::<u32>("generate", d, args.remove(0)));

      let [red, green, blue]: [u8; 3] =
        ["red", "green", "blue"].map(|c| parse_number::<u8>("generate", c, args.remove(0)));

      let mut imgbuf = ImageBuffer::new(width, height);

      for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
      }

      imgbuf.save(args.remove(0)).unwrap();
    }
    None => exit(1),
  }
}

/// Grayscales an image.
///
/// Arguments:
///
/// * `input` - String
/// * `output` - String
///
/// Usage: ```grayscale input.png output.png```
pub fn grayscale(args: &mut Vec<String>) {
  match check_for_invalid_args(&args, "grayscale", 2) {
    Some(()) => {
      let img = open_image(args.remove(0));

      // .grayscale() takes no arguments. It returns a new image.
      let new_image = img.grayscale();
      save_image(new_image, args.remove(0));
    }
    None => exit(1),
  }
}

/// Invert an image.
///
/// Arguments:
///
/// * `input` - String
/// * `output` - String
///
/// Usage: ```invert input.png output.png```
pub fn invert(args: &mut Vec<String>) {
  match check_for_invalid_args(&args, "invert", 2) {
    Some(()) => {
      let mut img = open_image(args.remove(0));

      // .invert() takes no arguments and converts the image in-place, so you
      // will use the same image to save out to a different file.
      img.invert();
      save_image(img, args.remove(0));
    }
    None => exit(1),
  }
}

/// Rotates an image.
///
/// Arguments:
///
/// * `rotate` - u32 (90, 180, 270)
/// * `input` - String
/// * `output` - String
///
/// Usage: ```rotate input.png output.png```
pub fn rotate(args: &mut Vec<String>) {
  match check_for_invalid_args(&args, "rotate", 3) {
    Some(()) => {
      let rotate = args.remove(0);
      let img = open_image(args.remove(0));

      let new_image = match rotate.as_str() {
        "90" => img.rotate90(),
        "180" => img.rotate180(),
        "270" => img.rotate270(),
        _ => img.rotate90(),
      };

      save_image(new_image, args.remove(0));
    }
    None => exit(1),
  }
}
