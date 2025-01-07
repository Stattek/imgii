use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
    sync::Arc,
    thread,
};

use image::{imageops, load_from_memory, DynamicImage, GenericImageView, ImageBuffer};
use regex::Regex;
use text_to_png::{Color, TextRenderer, TextToPngError};

// represents a colored character to write
struct ColoredStr {
    red: u8,
    blue: u8,
    green: u8,
    str: String,
}

const CHAR_SIZE: i32 = 16;

pub struct ImageData(ImageBuffer<image::Rgba<u8>, Vec<u8>>);

#[derive(Clone)]
pub struct MyImageWriter {
    pub imagebuf: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl MyImageWriter {
    /// Creates a new image writer containing a single image
    pub fn from_imagedata(the_image: &ImageData) -> Self {
        Self {
            imagebuf: the_image.0.clone(),
        }
    }

    /// Creates a new image writer from an image buffer
    pub fn from_imagebuffer(imagebuf: ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Self {
        Self { imagebuf }
    }

    /// Creates a new image writer with two images appended.
    pub fn new_append_right(left: &ImageData, right: &ImageData) -> Result<Self, ()> {
        let left_img = &left.0;
        let right_img = &right.0;

        let width = left_img.width() + right_img.width();
        let height = {
            if left_img.height() > right_img.height() {
                left_img.height()
            } else {
                right_img.height()
            }
        };

        let mut imgbuf = image::ImageBuffer::new(width, height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let new_pixel = {
                if left_img.in_bounds(x, y) {
                    // we are within the width of the left image
                    *left_img.get_pixel(x, y)
                } else if !x.overflowing_sub(left_img.width()).1
                    && right_img.in_bounds(x - left_img.width(), y)
                {
                    // we are beyond the width of the left image, so write the right image
                    let dst_x = x - left_img.width();
                    let dst_y = y;
                    *right_img.get_pixel(dst_x, dst_y)
                } else {
                    // we are beyond the width of either image, meaning that one has a larger height than the other.
                    image::Rgba([0, 0, 0, 0])
                }
            };
            // write the pixel we have chosen
            *pixel = new_pixel;
        }

        Ok(Self { imagebuf: imgbuf })
    }

    /// Appends an image to the right of the current image buffer.
    pub fn append_right(&mut self, right: &ImageData) -> Result<(), ()> {
        // load the image into memory
        let right_img = &right.0;

        let width = self.imagebuf.width() + right_img.width();
        let height = {
            if self.imagebuf.height() > right_img.height() {
                self.imagebuf.height()
            } else {
                right_img.height()
            }
        };

        let mut imgbuf = image::ImageBuffer::new(width, height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let new_pixel = {
                if self.imagebuf.in_bounds(x, y) {
                    // we are within the width of the left image
                    *self.imagebuf.get_pixel(x, y)
                } else if !x.overflowing_sub(self.imagebuf.width()).1
                    && right_img.in_bounds(x - self.imagebuf.width(), y)
                {
                    // we are beyond the width of the left image, so write the right image
                    let dst_x = x - self.imagebuf.width();
                    let dst_y = y;
                    *right_img.get_pixel(dst_x, dst_y)
                } else {
                    // we are beyond the width of either image, meaning that one has a larger height than the other.
                    image::Rgba([0, 0, 0, 0])
                }
            };
            // write the pixel we have chosen
            *pixel = new_pixel;
        }

        // save the new image buffer
        self.imagebuf = imgbuf;
        Ok(())
    }

    /// Appends an image to the bottom of the current image buffer.
    pub fn append_down(
        &mut self,
        bottom: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    ) -> Result<(), ()> {
        // load the image into memory

        let width = {
            if self.imagebuf.width() > bottom.width() {
                self.imagebuf.width()
            } else {
                bottom.width()
            }
        };
        let height = self.imagebuf.height() + bottom.height();

        let mut imgbuf = image::ImageBuffer::new(width, height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let new_pixel = {
                if self.imagebuf.in_bounds(x, y) {
                    // we are within the height of the left image
                    *self.imagebuf.get_pixel(x, y)
                } else if !y.overflowing_sub(self.imagebuf.height()).1
                    && bottom.in_bounds(x, y - self.imagebuf.height())
                {
                    // we are beyond the height of the top image, so write the bottom image
                    let dst_x = x;
                    let dst_y = y - self.imagebuf.height();
                    *bottom.get_pixel(dst_x, dst_y)
                } else {
                    // we are beyond the height of either image, meaning that one has a larger width than the other.
                    image::Rgba([0, 0, 0, 0])
                }
            };
            // write the pixel we have chosen
            *pixel = new_pixel;
        }

        // save the new image buffer
        self.imagebuf = imgbuf;
        Ok(())
    }

    /// Appends an image to the bottom of the current image buffer.
    pub fn new_append_down(
        top: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        bottom: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    ) -> Result<Self, ()> {
        // load the image into memory

        let width = {
            if top.width() > bottom.width() {
                top.width()
            } else {
                bottom.width()
            }
        };
        let height = top.height() + bottom.height();

        let mut imgbuf = image::ImageBuffer::new(width, height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let new_pixel = {
                if top.in_bounds(x, y) {
                    // we are within the height of the left image
                    *top.get_pixel(x, y)
                } else if !y.overflowing_sub(top.height()).1
                    && bottom.in_bounds(x, y - top.height())
                {
                    // check that we don't have an overflow and
                    // we are beyond the height of the top image, so write the bottom image
                    let dst_x = x;
                    let dst_y = y - top.height();
                    *bottom.get_pixel(dst_x, dst_y)
                } else {
                    // we are beyond the height of either image, meaning that one has a larger width than the other.
                    image::Rgba([0, 0, 0, 0])
                }
            };
            // write the pixel we have chosen
            *pixel = new_pixel;
        }

        // save the new image buffer
        Ok(Self { imagebuf: imgbuf })
    }
}

/// Converts string data into a png
fn str_to_png(data: ColoredStr) -> Result<ImageData, TextToPngError> {
    let renderer = TextRenderer::default();
    let text_png = renderer.render_text_to_png_data(
        data.str,
        CHAR_SIZE,
        Color::new(data.red, data.green, data.blue),
    )?;

    // we can manually read the data from this generated text image into another library `image`
    let mut loaded_img = load_from_memory(&text_png.data).unwrap();
    loaded_img =
        loaded_img.resize_exact((CHAR_SIZE / 2) as u32, CHAR_SIZE as u32, imageops::Nearest);

    Ok(ImageData(loaded_img.into_rgba8()))
}

/// Creates a transparent png in place of a character
fn str_to_transparent_png() -> ImageData {
    let image = DynamicImage::new_rgba8((CHAR_SIZE / 2) as u32, CHAR_SIZE as u32);
    ImageData(image.into_rgba8())
}

/// The general idea:
/// Use regex to find the rgb values for each character then print each character into its own image
/// Then, from each image that is created, we horizontally merge the character images to form a line of text
/// Finally, from each image containing a line of text, we should vertically merge the images to form a whole image of converted ascii to text.
fn convert_ascii_to_png(input_file_name: String, output_file_name: String) {
    let infile = File::open(&input_file_name).unwrap();
    let reader = BufReader::new(infile);

    // contains lines of images
    // starting at 0 is the top, first line of the vector
    // inside an inner vec, 0 starts at the leftmost character of the line
    let mut lines = vec![];
    // read every line in the file
    for line in reader.lines().flatten() {
        let mut char_images = vec![];
        // we need to find each character that we are going to write
        // we assume that there's only one character for each color
        let pattern = r"\[38;2;([0-9]+);([0-9]+);([0-9]+)m(.)";
        let _control_char = '\u{1b}'; // another way to represent the ansi escape character `\033`
        let re = Regex::new(pattern).unwrap();
        // create the image for this character
        for (_str, [r, g, b, the_str]) in re.captures_iter(&line).map(|c| c.extract()) {
            let red = r.parse::<u8>().unwrap();
            let green = g.parse::<u8>().unwrap();
            let blue = b.parse::<u8>().unwrap();

            let generated_png = {
                if the_str.trim().is_empty() {
                    // create a transparent png for a space
                    str_to_transparent_png()
                } else {
                    // render the actual text if it's not empty
                    let colored = ColoredStr {
                        red,
                        green,
                        blue,
                        str: String::from(the_str),
                    };

                    str_to_png(colored).unwrap()
                }
            };

            char_images.push(generated_png);
        }

        lines.push(char_images);
    }

    let mut image_writers = vec![];
    for line in lines {
        let image_writer = {
            if line.len() >= 2 {
                // if the current line is at least two images, then we append them together and then the rest of them
                let mut the_writer = MyImageWriter::new_append_right(&line[0], &line[1]).unwrap();

                for i in 2..line.len() {
                    // append all of the images in this line
                    the_writer.append_right(&line[i]).unwrap();
                }

                the_writer
            } else if line.len() > 0 {
                MyImageWriter::from_imagedata(&line[0])
            } else {
                // we don't have anything to write
                eprintln!("WARNING: Skipped an empty line of images");
                continue;
            }
        };

        image_writers.push(image_writer);
    }

    let final_image_writer: Option<MyImageWriter> = {
        if image_writers.len() >= 2 {
            // if the current line is at least two images, then we append them together and then the rest of them
            let mut the_writer = MyImageWriter::new_append_down(
                &image_writers[0].imagebuf,
                &image_writers[1].imagebuf,
            )
            .unwrap();

            for i in 2..image_writers.len() {
                // append all of the images in this line
                the_writer.append_down(&image_writers[i].imagebuf).unwrap();
            }

            Some(the_writer)
        } else if image_writers.len() > 0 {
            Some(MyImageWriter::from_imagebuffer(
                image_writers[0].imagebuf.clone(),
            ))
        } else {
            // we don't have anything to write
            None
        }
    };

    match final_image_writer {
        Some(writer) => {
            writer.imagebuf.save(&output_file_name).unwrap();
            println!("Saved PNG {}", output_file_name);
        }
        None => {
            panic!("Could not save the image!");
        }
    }
}

fn main() {
    let pool = threadpool::ThreadPool::new(num_cpus::get());
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: ascii_to_png <input_name_format> <output_name_format> <OPTIONAL: final_image_index>"
        );
        exit(1); // error
    }
    let input_name_format = Arc::new(args[1].clone());
    let output_name_format = Arc::new(args[2].clone());

    let final_image_index: u32 = {
        if args.len() > 3 {
            args[3]
                .parse()
                .expect("Could not convert num_inputs to u32")
        } else {
            1
        }
    };

    for i in 1..=final_image_index {
        let copy_input_name_format = Arc::clone(&input_name_format);
        let copy_output_name_format = Arc::clone(&output_name_format);
        pool.execute(move || {
            // convert to ascii before performing the conversion
            let input_file_name = copy_input_name_format.replace("%d", i.to_string().as_str());
            let output_file_name = copy_output_name_format.replace("%d", i.to_string().as_str());
            convert_ascii_to_png(input_file_name, output_file_name);
        });
    }

    pool.join();
    println!("---Success!---")
}
