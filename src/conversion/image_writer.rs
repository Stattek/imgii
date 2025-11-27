use crate::{
    conversion::{
        converters::generic_converter::Imgii2dImage,
        image_data::{ImageData, InternalImage},
        render_char_to_png::calculate_char_dimensions,
    },
    error::{ImgiiError, InvalidParameterError, ParseImageError},
    options::ImgiiOptions,
};
use rayon::prelude::*;

/// An image writer which holds a rendered ASCII image.
#[derive(Debug, Clone)]
pub(crate) struct AsciiImageWriter {
    pub(crate) imagebuf: ImageData,
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
    pub(crate) fn from_2d_vec(
        the_image: Imgii2dImage,
        pngii_options: &ImgiiOptions,
    ) -> Result<Self, ImgiiError> {
        if the_image.image_2d.is_empty() {
            // no image to build
            return Err(InvalidParameterError::new(String::from("parts")).into());
        }

        // find out the new canvas size
        // this should always exist
        let char_width = the_image.image_2d[0].as_buffer().width();
        let char_height = the_image.image_2d[0].as_buffer().height();

        // calculate image resolution in pixels based on this reference image
        let height = char_height * the_image.height as u32;
        let width = char_width * the_image.width as u32;

        // create the new canvas to write to
        let mut canvas: InternalImage = image::ImageBuffer::new(width, height);

        // copy over pixels to canvas
        canvas.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            // the index into the row and column from the image_2d vec
            let row = y / char_height;
            let column = x / char_width;

            // the index into the inner image that we want to read from
            let inner_x = x % char_width;
            let inner_y = y % char_height;

            let new_pixel = the_image.image_2d
                [column as usize + row as usize * the_image.width as usize]
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
