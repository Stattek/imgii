use crate::{
    ImgiiOptions,
    conversion::{image_data::ImageData, render_char_to_png::str_to_png},
    error::{FontError, ImgiiError, InternalError, ParseIntError},
};

use super::super::render_char_to_png::{ColoredStr, str_to_transparent_png};
use ab_glyph::FontRef;
use regex::Regex;

// TODO: Read this font at runtime instead and allow the user to choose

// read bytes for the font
const FONT_FILE: &str = "../../../fonts/UbuntuMono.ttf";
const FONT_BYTES: &[u8] = include_bytes!("../../../fonts/UbuntuMono.ttf");

/// Simple struct for holding a 2d image with its width and height.
#[derive(Clone, Debug)]
pub(crate) struct Imgii2dImage {
    pub image_2d: Vec<ImageData>,
    pub width: usize,
    pub height: usize,
}

/// Generic function for parsing and rendering ASCII into an image.
///
/// * `imgii_options`: The imgii options for rendering ASCII.
/// * `ascii_text`: The ASCII text to render.
///
/// # Returns
/// `Ok` containing a 2d `Vec` if `ImageData`, holding each character image, otherwise an `Err`.
pub(crate) fn render_ascii_generic(
    imgii_options: &ImgiiOptions,
    ascii_text: String,
) -> Result<Imgii2dImage, ImgiiError> {
    // set up font for rendering
    let font = FontRef::try_from_slice(FONT_BYTES)
        // there's nothing useful in this error, convert it!
        .map_err(|_| FontError::new(String::from(FONT_FILE)))?;

    // 2d Vec of images for each character
    let mut image_2d_vec = Vec::new();

    // create this once since it will always be the same
    let transparent_png = str_to_transparent_png(imgii_options);

    // width and height, in characters
    // NOTE: we can know height beforehand but we have to wait until we have parsed a whole line of
    // text to know the width
    let (mut width, mut height) = (0, ascii_text.lines().count());

    // read every line in the file
    for (i, line) in ascii_text.lines().enumerate() {
        // we need to find each character that we are going to write
        // we assume that there's only one character for each color
        // NOTE: \u{1b} represents the \033 character
        let pattern_str = concat!('\u{1b}', r"\[38;2;([0-9]+);([0-9]+);([0-9]+)m(.)");

        // TODO: if multiple threads are using this same regex object, maybe we could make it a
        // static global or compile it early so we can reuse it? Maybe as a "parser" object?
        let re = Regex::new(pattern_str)?;

        // current line's width
        let mut line_width = 0;

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
                    transparent_png.clone()
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

            line_width += 1;
            image_2d_vec.push(generated_png);
        }

        // check that this width is always the same now that we have the width
        if i != 0 {
            assert_eq!(
                width, line_width,
                "width {} is not equal to the current line width {}",
                width, line_width
            );
        } else {
            width = line_width;
            // now we can reserve the rest of the space for our vec
            image_2d_vec.reserve(width * height);
        }
    }

    assert!(
        width * height == image_2d_vec.len(),
        "expected length of the 2d vector was {} but got {}",
        width * height,
        image_2d_vec.len()
    );

    Ok(Imgii2dImage {
        image_2d: image_2d_vec,
        width: width,
        height: height,
    })
}
