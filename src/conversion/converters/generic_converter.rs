use std::{collections::HashMap, sync::Arc};

use crate::{
    ImgiiOptions,
    conversion::{image_data::ImageData, render_char_to_png::str_to_png},
    error::{FontError, ImgiiError, ParseIntError, RenderError},
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
    pub(crate) image_2d: Vec<Arc<ImageData>>,
    pub(crate) width: usize,
    pub(crate) height: usize,
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

    // width and height, in characters
    // NOTE: we can know height beforehand but we have to wait until we have parsed a whole line of
    // text to know the width
    let (mut width, height) = (0, ascii_text.lines().count());

    // hold already rendered images so we don't have to render them more than once! Rendering is
    // slow
    let mut rendered_images: HashMap<ColoredStr, Arc<ImageData>> = HashMap::new();
    // create transparent image once since it will always be the same
    let transparent_png = Arc::from(str_to_transparent_png(imgii_options));

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

                    // check if this image was already rendered before
                    let rendered_img = rendered_images.get(&colored);
                    match rendered_img {
                        // we have rendered this image before, so clone it
                        Some(rendered_img) => rendered_img.clone(),
                        None => {
                            // we haven't rendered this image before, so render it
                            let image_data = Arc::from(str_to_png(&colored, &font, imgii_options));
                            let result = rendered_images.insert(colored, image_data.clone());
                            if result.is_some() {
                                return Err(RenderError::new(String::from(
                                    "this image should not exist already in the hash map",
                                ))
                                .into());
                            }
                            image_data
                        }
                    }
                }
            };

            line_width += 1;
            image_2d_vec.push(generated_png);
        }

        if i == 0 {
            // get the width of the entire image. This should always be the same
            width = line_width;
            // now we can reserve the rest of the capacity we need for our vec
            // NOTE: this can panic if the vec is too large
            image_2d_vec.reserve(width * height);
        } else {
            // check that this width is always the same now that we have the width
            if width != line_width {
                return Err(RenderError::new(format!(
                    "width {} is not equal to the current line width {}",
                    width, line_width
                ))
                .into());
            }
        }
    }

    // Check that the length of the final vector is what we expect. If not, something has gone
    // terribly wrong, and we should not continue.
    if width * height != image_2d_vec.len() {
        return Err(RenderError::new(format!(
            "expected length of the 2d vector was {} but got {}",
            width * height,
            image_2d_vec.len()
        ))
        .into());
    }

    Ok(Imgii2dImage {
        image_2d: image_2d_vec,
        width,
        height,
    })
}
