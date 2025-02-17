use crate::{
    image_data::ImageData, render_char_to_png::calculate_char_dimensions, AsciiImageOptions,
};
use image::GenericImageView;
use rayon::prelude::*;

#[derive(Clone)]
pub struct AsciiImageWriter {
    pub imagebuf: ImageData,
}

impl AsciiImageWriter {
    /// Creates a new image writer containing a single image
    #[allow(unused)]
    pub fn from_imagedata(the_image: ImageData) -> Self {
        Self {
            imagebuf: the_image,
        }
    }

    /// Creates a new image writer with two images appended.
    /// NOTE: Very expensive when used in succession. Only use in special circumstances.
    #[allow(unused)]
    pub fn new_append_right(left: &ImageData, right: &ImageData) -> Self {
        let width = left.width() + right.width();
        let height = {
            if left.height() > right.height() {
                left.height()
            } else {
                right.height()
            }
        };

        let mut imgbuf = image::ImageBuffer::new(width, height);
        imgbuf.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let new_pixel = {
                if left.in_bounds(x, y) {
                    // we are within the width of the left image
                    *left.get_pixel(x, y)
                } else if !x.overflowing_sub(left.width()).1 && right.in_bounds(x - left.width(), y)
                {
                    // we are beyond the width of the left image, so write the right image
                    let dst_x = x - left.width();
                    let dst_y = y;
                    *right.get_pixel(dst_x, dst_y)
                } else {
                    // we are beyond the width of either image, meaning that one has a larger height than the other.
                    image::Rgba([0, 0, 0, 0])
                }
            };
            // write the pixel we have chosen
            *pixel = new_pixel;
        });

        Self {
            imagebuf: ImageData::new(imgbuf),
        }
    }

    /// Appends an image to the right of the current image buffer.
    /// NOTE: Very expensive when used in succession. Only use in special circumstances.
    #[allow(unused)]
    pub fn append_right(&mut self, right: &ImageData) {
        let width = self.imagebuf.width() + right.width();
        let height = {
            if self.imagebuf.height() > right.height() {
                self.imagebuf.height()
            } else {
                right.height()
            }
        };

        let mut imgbuf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::new(width, height);
        imgbuf.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let new_pixel = {
                if self.imagebuf.in_bounds(x, y) {
                    // we are within the width of the left image
                    *self.imagebuf.get_pixel(x, y)
                } else if !x.overflowing_sub(self.imagebuf.width()).1
                    && right.in_bounds(x - self.imagebuf.width(), y)
                {
                    // we are beyond the width of the left image, so write the right image
                    let dst_x = x - self.imagebuf.width();
                    let dst_y = y;
                    *right.get_pixel(dst_x, dst_y)
                } else {
                    // we are beyond the width of either image, meaning that one has a larger height than the other.
                    image::Rgba([0, 0, 0, 0])
                }
            };
            // write the pixel we have chosen
            *pixel = new_pixel;
        });

        // save the new image buffer
        self.imagebuf = ImageData::new(imgbuf);
    }

    /// Appends an image to the bottom of the current image buffer.
    /// NOTE: Very expensive when used in succession. Only use in special circumstances.
    #[allow(unused)]
    pub fn append_down(&mut self, bottom: &ImageData) {
        let width = {
            if self.imagebuf.width() > bottom.width() {
                self.imagebuf.width()
            } else {
                bottom.width()
            }
        };
        let height = self.imagebuf.height() + bottom.height();

        let mut imgbuf = image::ImageBuffer::new(width, height);
        imgbuf.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
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
        });

        // save the new image buffer
        self.imagebuf = ImageData::new(imgbuf);
    }

    /// Appends an image to the bottom of the current image buffer.
    /// NOTE: Very expensive when used in succession. Only use in special circumstances.
    #[allow(unused)]
    pub fn new_append_down(top: &ImageData, bottom: &ImageData) -> Self {
        let width = {
            if top.width() > bottom.width() {
                top.width()
            } else {
                bottom.width()
            }
        };
        let height = top.height() + bottom.height();

        let mut imgbuf = image::ImageBuffer::new(width, height);
        imgbuf.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
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
        });

        // save the new image buffer
        Self {
            imagebuf: ImageData::new(imgbuf),
        }
    }

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
    pub fn from_2d_vec(
        parts: Vec<Vec<ImageData>>,
        ascii_image_options: &AsciiImageOptions,
    ) -> Option<Self> {
        if parts.is_empty() || parts[0].is_empty() {
            return None; // no image to build
        }

        let font_size = ascii_image_options.get_font_size();

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
            let row = y / char_height as u32;
            let column = x / char_width as u32;

            // the index into the inner image that we want to read from
            let inner_x = x % char_width as u32;
            let inner_y = y % char_height as u32;

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
