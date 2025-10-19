use crate::{conversion::image_data::ImageData, options::ImgiiOptions};
use ab_glyph::{FontRef, PxScale};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rayon::prelude::*;

/// Represents a colored string to write.
/// All characters are contiguous and share the same color.
#[derive(Debug, Clone)]
pub struct ColoredStr {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
    pub string: String,
}

const BACKGROUND_PIXEL: Rgba<u8> = Rgba([0, 0, 0, u8::MAX]);

/// Converts string data into a png
/// Uses `imageproc` to render text.
pub fn str_to_png(data: ColoredStr, font: &FontRef<'_>, imgii_options: &ImgiiOptions) -> ImageData {
    let font_size = imgii_options.font_size();
    let (char_width, char_height) = calculate_char_dimensions(font_size);
    // create our image to work with
    let mut image = RgbaImage::new(char_width, char_height);
    let scale = PxScale {
        x: font_size as f32,
        y: font_size as f32,
    };

    // set background if user wants it
    if imgii_options.background() {
        set_background(&mut image);
    }

    draw_text_mut(
        &mut image,
        Rgba([data.red, data.green, data.blue, u8::MAX]),
        0,
        0,
        scale,
        &font,
        &data.string,
    );

    ImageData::new(image)
}

// PERF: this is a costly operation and should probably be removed
fn set_background(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    image.par_enumerate_pixels_mut().for_each(|(_, _, pixel)| {
        // set background
        *pixel = BACKGROUND_PIXEL;
    });
}

/// Creates a transparent png in place of a character
pub fn str_to_transparent_png(imgii_options: &ImgiiOptions) -> ImageData {
    let (char_width, char_height) = calculate_char_dimensions(imgii_options.font_size());
    let mut output = DynamicImage::new_rgba8(char_width, char_height).into();

    // TODO: instead of doing a background like this, why don't we create a single image that is a
    // solid color (or we could do more interesting backgrounds) and overlay the output image over
    // top of that?

    // set background if user wants it
    if imgii_options.background() {
        set_background(&mut output);
    }

    ImageData::new(output)
}

/// Calculates character dimensions are returns them
///
/// # Returns
/// (width, height) in a tuple
pub fn calculate_char_dimensions(font_size: u32) -> (u32, u32) {
    (font_size / 2, font_size)
}
