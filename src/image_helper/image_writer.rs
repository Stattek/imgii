use crate::{
    ImgiiOptions,
    error::{ImgiiError, InvalidParameterError, ParseImageError},
    image_helper::{
        image_data::{ImageData, InternalImage},
        render_char_to_png::calculate_char_dimensions,
    },
};
use rayon::prelude::*;

/// An image writer which holds a rendered ASCII image.
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
    /// Builds a new image from a 2d `Vec` of image parts. Stitches an image together from a 2D
    /// vector, converting the 2D vector into a single image.
    ///
    /// # Params
    /// - `parts` - A 2d `Vec` of images, with the `parts` array containing the rows (starting from 0
    ///   as the top of the image) and the inner array containing the columns (starting from 0 as
    ///   the leftmost part of the image).
    ///
    /// # Returns
    /// - An `Option` containing `Some` `AsciiImageWriter` upon success, or a
    ///   `None` upon failure.
    #[must_use]
    pub fn from_2d_vec(
        parts: Vec<Vec<ImageData>>,
        pngii_options: &ImgiiOptions,
    ) -> Result<Self, ImgiiError> {
        if parts.is_empty() || parts[0].is_empty() {
            // no image to build
            return Err(InvalidParameterError::new(String::from("parts")).into());
        }

        let font_size = pngii_options.font_size();

        let (mut height, mut width) = (0, 0);
        // find out the new canvas size
        for (cur_line, line) in parts.iter().enumerate() {
            // assume that every image has the same height and width
            if !line.is_empty() {
                height += line[0].as_buffer().height();
                // calculate the width
                width = line[0].as_buffer().width() * line.len() as u32;
            } else {
                return Err(ParseImageError::new(cur_line).into());
            }
        }

        // create the new canvas to write to
        let mut canvas: InternalImage = image::ImageBuffer::new(width, height);

        // calculate character width and height
        let (char_width, char_height) = calculate_char_dimensions(font_size);

        canvas.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            // the index into the row and column from the parts vec
            let row = y / char_height;
            let column = x / char_width;

            // the index into the inner image that we want to read from
            let inner_x = x % char_width;
            let inner_y = y % char_height;

            let new_pixel = parts[row as usize][column as usize]
                .as_buffer()
                .get_pixel(inner_x, inner_y);
            // write the pixel we have chosen
            *pixel = *new_pixel;
        });

        // save the new image buffer
        Ok(Self {
            imagebuf: ImageData::new(canvas),
        })
    }
}
