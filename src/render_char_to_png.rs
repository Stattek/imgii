use image::{imageops, load_from_memory, DynamicImage};
use text_to_png::{Color, TextRenderer};

use crate::image_data::ImageData;

/// Represents a colored string to write.
/// All characters are contiguous and share the same color.
#[derive(Debug)]
pub struct ColoredStr {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
    pub string: String,
}

const CHAR_FONT_SIZE: i32 = 16;

/// Converts string data into a png
pub fn str_to_png(data: ColoredStr) -> Result<ImageData, ()> {
    let renderer = TextRenderer::default();
    let text_png = renderer.render_text_to_png_data(
        data.string,
        CHAR_FONT_SIZE,
        Color::new(data.red, data.green, data.blue),
    );

    match text_png {
        Ok(text_png_val) => {
            let loaded_img = load_from_memory(&text_png_val.data);
            match loaded_img {
                Ok(mut loaded_img_val) => {
                    loaded_img_val = loaded_img_val.resize_exact(
                        (CHAR_FONT_SIZE / 2) as u32,
                        CHAR_FONT_SIZE as u32,
                        imageops::Nearest,
                    );
                    // we can manually read the data from this generated text image into another library `image`
                    Ok(ImageData::new(loaded_img_val.into_rgba8()))
                }
                Err(_) => {
                    return Err(());
                }
            }
        }
        Err(_) => {
            return Err(());
        }
    }
}

/// Creates a transparent png in place of a character
/// FUTURE: if this is made to support multiple characters per color, take input for number of characters and make the transparent image based off of that.
pub fn str_to_transparent_png() -> ImageData {
    ImageData::new(
        DynamicImage::new_rgba8((CHAR_FONT_SIZE / 2) as u32, CHAR_FONT_SIZE as u32).into_rgba8(),
    )
}
