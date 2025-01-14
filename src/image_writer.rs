use crate::image_data::ImageData;
use image::GenericImageView;
#[derive(Clone)]
pub struct MyImageWriter {
    pub imagebuf: ImageData,
}

impl MyImageWriter {
    /// Creates a new image writer containing a single image
    pub fn from_imagedata(the_image: ImageData) -> Self {
        Self {
            imagebuf: the_image,
        }
    }

    /// Creates a new image writer with two images appended.
    /// NOTE: Very expensive.
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
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
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
        }

        Self {
            imagebuf: ImageData::new(imgbuf),
        }
    }

    /// Appends an image to the right of the current image buffer.
    /// NOTE: Very expensive.
    pub fn append_right(&mut self, right: &ImageData) {
        let width = self.imagebuf.width() + right.width();
        let height = {
            if self.imagebuf.height() > right.height() {
                self.imagebuf.height()
            } else {
                right.height()
            }
        };

        let mut imgbuf = image::ImageBuffer::new(width, height);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
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
        }

        // save the new image buffer
        self.imagebuf = ImageData::new(imgbuf);
    }

    /// Appends an image to the bottom of the current image buffer.
    /// NOTE: Very expensive.
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
        self.imagebuf = ImageData::new(imgbuf);
    }

    /// Appends an image to the bottom of the current image buffer.
    /// NOTE: Very expensive.
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
        Self {
            imagebuf: ImageData::new(imgbuf),
        }
    }
}
