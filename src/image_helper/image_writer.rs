use crate::{
    ImgiiOptions,
    image_helper::{image_data::ImageData, render_char_to_png::calculate_char_dimensions},
};
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct AsciiImageWriter {
    pub imagebuf: ImageData,
}

impl From<ImageData> for AsciiImageWriter {
    /// Creates an image writer from image data.
    fn from(the_image: ImageData) -> Self {
        Self {
            imagebuf: the_image,
        }
    }
}

impl AsciiImageWriter {
    /// Builds a new image from a 2d `Vec` of image parts.
    ///
    /// # Params
    /// - `parts` - A 2d `Vec` of images, with the `parts` array containing the rows (starting from 0
    /// as the top of the image) and the inner array containing the columns (starting from 0 as
    /// the leftmost part of the image).
    ///
    /// # Returns
    /// - An `Option` containing `Some` `AsciiImageWriter` upon success, or a
    /// `None` upon failure.
    pub fn from_2d_vec(parts: Vec<Vec<ImageData>>, pngii_options: &ImgiiOptions) -> Option<Self> {
        if parts.is_empty() || parts[0].is_empty() {
            return None; // no image to build
        }

        let font_size = pngii_options.font_size();

        let (mut height, mut width) = (0, 0);
        // find out the new canvas size
        let mut cur_line = 0;
        for line in &parts {
            // assume that every image has the same height and width
            if !line.is_empty() {
                height += line[0].height();
                // calculate the width
                width = line[0].width() * line.len() as u32;
            } else {
                eprintln!(
                    "Skipped an empty line of images at line {}! Malformed data?",
                    cur_line
                );
                return None;
            }

            cur_line += 1;
        }

        // create the new canvas to write to
        let mut canvas: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::new(width, height);

        // calculate character width and height
        let (char_width, char_height) = calculate_char_dimensions(font_size);

        canvas.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            // the index into the row and column from the parts vec
            let row = y / char_height;
            let column = x / char_width;

            // the index into the inner image that we want to read from
            let inner_x = x % char_width;
            let inner_y = y % char_height;

            let new_pixel = parts[row as usize][column as usize].get_pixel(inner_x, inner_y);
            // write the pixel we have chosen
            *pixel = *new_pixel;
        });

        // save the new image buffer
        Some(Self {
            imagebuf: ImageData::new(canvas),
        })
    }
}
