use crate::image_data::ImageData;
use ab_glyph::{FontRef, PxScale};
use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use std::u8;

/// Represents a colored string to write.
/// All characters are contiguous and share the same color.
#[derive(Debug)]
pub struct ColoredStr {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
    pub string: String,
}

pub const DEFAULT_CHAR_FONT_SIZE: u32 = 16;

/// Converts string data into a png
/// Uses `imageproc` to render text.
pub fn str_to_png(data: ColoredStr, font: &FontRef<'_>, font_size: u32) -> Result<ImageData, ()> {
    let (char_width, char_height) = calculate_char_dimensions(font_size);
    // create our image to work with
    let mut image = RgbaImage::new(char_width, char_height);
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
    let (w, h) = text_size(scale, &font, &data.string);
    println!("text size: w={}, h={}", w, h);

    return Ok(ImageData::new(image));
}

/// Creates a transparent png in place of a character
pub fn str_to_transparent_png(font_size: u32) -> ImageData {
    let (char_width, char_height) = calculate_char_dimensions(font_size);
    ImageData::new(DynamicImage::new_rgba8(char_width, char_height).into())
}

/// Calculates character dimensions are returns them
///
/// # Returns
/// (width, height) in a tuple
pub fn calculate_char_dimensions(font_size: u32) -> (u32, u32) {
    (font_size / 2, font_size)
}
