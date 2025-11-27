use crate::{conversion::image_data::ImageData, options::ImgiiOptions};
use ab_glyph::{FontRef, PxScale};
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;

/// Represents a colored string to write.
/// All characters are contiguous and share the same color.
#[derive(Debug, Clone)]
pub(crate) struct ColoredStr {
    pub(crate) red: u8,
    pub(crate) blue: u8,
    pub(crate) green: u8,
    pub(crate) string: String,
}

const BACKGROUND_PIXEL: Rgba<u8> = Rgba([0, 0, 0, u8::MAX]);

/// Converts string data into a png.
/// Uses `imageproc` to render text.
pub(crate) fn str_to_png(
    data: ColoredStr,
    font: &FontRef<'_>,
    imgii_options: &ImgiiOptions,
) -> ImageData {
    let font_size = imgii_options.font_size();
    let (char_width, char_height) = calculate_char_dimensions(font_size);
    // create our image to work with
    let mut image = if imgii_options.background() {
        // create with background
        ImageBuffer::from_pixel(char_width, char_height, BACKGROUND_PIXEL)
    } else {
        ImageBuffer::new(char_width, char_height)
    };
    let scale = PxScale {
        x: font_size as f32,
        y: font_size as f32,
    };

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

/// Creates a transparent png in place of a character
pub(crate) fn str_to_transparent_png(imgii_options: &ImgiiOptions) -> ImageData {
    let (char_width, char_height) = calculate_char_dimensions(imgii_options.font_size());
    let output = if imgii_options.background() {
        // create image with background
        ImageBuffer::from_pixel(char_width, char_height, BACKGROUND_PIXEL)
    } else {
        // empty image
        ImageBuffer::new(char_width, char_height)
    };

    ImageData::new(output)
}

/// Calculates character dimensions are returns them
///
/// # Returns
/// (width, height) in a tuple
pub(crate) fn calculate_char_dimensions(font_size: u32) -> (u32, u32) {
    (font_size / 2, font_size)
}
