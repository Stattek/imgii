use crate::{
    ImgiiOptions,
    conversion::{image_data::ImageData, render_char_to_png::str_to_png},
    error::{FontError, ImgiiError, ParseIntError},
};

use super::super::render_char_to_png::{ColoredStr, str_to_transparent_png};
use ab_glyph::FontRef;
use regex::Regex;

// TODO: Read this font at runtime instead and allow the user to choose

// read bytes for the font
const FONT_FILE: &str = "../../../fonts/UbuntuMono.ttf";
const FONT_BYTES: &[u8] = include_bytes!("../../../fonts/UbuntuMono.ttf");

/// Generic function for parsing and rendering ASCII into an image.
///
/// * `imgii_options`: The imgii options for rendering ASCII.
/// * `ascii_text`: The ASCII text to render.
pub fn render_ascii_generic(
    imgii_options: &ImgiiOptions,
    ascii_text: String,
) -> Result<Vec<Vec<ImageData>>, ImgiiError> {
    // set up font for rendering
    let font = FontRef::try_from_slice(FONT_BYTES)
        // there's nothing useful in this error, convert it!
        .map_err(|_| FontError::new(String::from(FONT_FILE)))?;

    // contains lines of images
    // starting at 0 is the top, first line of the vector
    // inside an inner vec, 0 starts at the leftmost character of the line
    let mut image_2d_vec = vec![];

    // read every line in the file
    for line in ascii_text.lines() {
        let mut char_images = vec![];

        // we need to find each character that we are going to write
        // we assume that there's only one character for each color
        let control_char = '\u{1b}'; // represents the ansi escape character `\033`
        let mut pattern_string = String::from(control_char);
        let pattern = r"\[38;2;([0-9]+);([0-9]+);([0-9]+)m(.)";
        pattern_string += pattern;

        // TODO: if multiple threads are using this same regex object, maybe we could make it a
        // static global or compile it early so we can reuse it? Maybe as a "parser" object?
        let re = Regex::new(&pattern_string)?;

        // create the image for this character
        for (_full_str, [r, g, b, the_str]) in re.captures_iter(line).map(|c| c.extract()) {
            let red = r.parse::<u8>().map_err(|err| {
                ParseIntError::new(String::from("red"), String::from(the_str), err)
            })?;
            let green = g.parse::<u8>().map_err(|err| {
                ParseIntError::new(String::from("green"), String::from(the_str), err)
            })?;
            let blue = b.parse::<u8>().map_err(|err| {
                ParseIntError::new(String::from("blue"), String::from(the_str), err)
            })?;

            let generated_png = {
                if the_str.trim().is_empty() {
                    // create a transparent png for a space
                    str_to_transparent_png(imgii_options)
                } else {
                    // render the actual text if it's not empty
                    let colored = ColoredStr {
                        red,
                        green,
                        blue,
                        string: String::from(the_str),
                    };

                    str_to_png(colored, &font, imgii_options)
                }
            };

            char_images.push(generated_png);
        }

        image_2d_vec.push(char_images);
    }

    Ok(image_2d_vec)
}
