use std::{fs::File, io::BufReader, sync::Mutex};

use crate::{
    PngiiOptions,
    image_helper::{image_data::ImageData, render_char_to_png::str_to_png},
};

use super::render_char_to_png::{ColoredStr, str_to_transparent_png};
use ab_glyph::FontRef;
use image::{AnimationDecoder, Delay, DynamicImage, Frame, codecs::gif::GifDecoder, open};
use rascii_art::{RenderOptions, render_image_to};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

// read bytes for the font
const FONT_BYTES: &[u8] = include_bytes!("../../fonts/UbuntuMono.ttf");

/// Holds the metadata for a frame that has been deconstructed.
///
/// * `image`: The image for this frame.
/// * `left`: The left value for this frame.
/// * `top`: The top value for this frame.
/// * `delay`: The delay for this frame.
/// * `idx`: The index of this frame. Saved for handling out of order frames due to multithreading.
pub struct FrameMetadata {
    pub left: u32,
    pub top: u32,
    pub delay: Delay,
    pub idx: usize,
}

impl FrameMetadata {
    /// Creates a new FrameMetadata.
    fn new(left: u32, top: u32, delay: Delay, idx: usize) -> Self {
        Self {
            left,
            top,
            delay,
            idx,
        }
    }
}

// TODO: document
/// Reads a GIF into a `Vec<DynamicImage>` for use with converting to ASCII.
///
/// # Params
/// * `input_file_name`: String slice containing the input file name.
///
/// # Returns
/// `Err()` upon error reading the GIF, `Ok()` otherwise.
fn read_gif(input_file_name: &str) -> Result<Vec<(DynamicImage, FrameMetadata)>, std::io::Error> {
    let file_in = BufReader::new(File::open(input_file_name)?);
    let decoder =
        GifDecoder::new(file_in).expect(format!("Could not read gif {}", input_file_name).as_str());

    let mut cur_idx = 0 as usize;
    // decode all of the frames of the gif and then convert each frame into a DynamicImage
    let frames = decoder
        .into_frames()
        .collect_frames()
        .expect(format!("Could not decode gif {}", input_file_name).as_str())
        .into_iter()
        .map(|value| {
            let idx = cur_idx;
            let left = value.left();
            let top = value.top();
            let delay = value.delay();
            cur_idx += 1; // increment the current index
            (
                // we split this from the frame metadata because we will not want the original image once we have converted it to ASCII
                value.into_buffer().into(),
                FrameMetadata::new(left, top, delay, idx),
            )
        })
        .collect();

    Ok(frames)
}

/// Reads the image as an ASCII string using `RASCII`.
///
/// # Params
/// * `input_file_name`: The input file name of the image to convert.
/// * `rascii_options`: The RASCII image options.
///
/// # Returns
/// * `String` containing the colored image data as ASCII, colored using terminal escape sequences.
fn read_image_as_ascii(input_file_name: &str, rascii_options: &RenderOptions) -> String {
    // render the ascii text with RASCII
    let mut ascii_text = String::new();
    let loaded_img =
        open(input_file_name).expect(format!("Could not open file ({})", input_file_name).as_str());
    render_image_to(&loaded_img, &mut ascii_text, &rascii_options)
        .expect("Error converting image to ASCII");

    ascii_text
}

/// Reads a gif as a list of ASCII strings
/// TODO: documentation
fn read_gif_as_ascii(
    input_file_name: &str,
    rascii_options: &RenderOptions,
) -> Vec<(String, FrameMetadata)> {
    // render the ascii text with RASCII
    let gif_images = read_gif(input_file_name)
        .expect(format!("Could not read gif {}", input_file_name).as_str());

    // PERF: check rayon performance vs. threadpool for gif
    // TODO: this data can probably be out of order due to multithreading, we might need to sort on
    // `idx`
    gif_images
        .into_par_iter()
        .map(|(image, deconstructed_frame)| {
            let mut ascii_text = String::new();
            render_image_to(&image, &mut ascii_text, rascii_options);
            (ascii_text, deconstructed_frame)
        })
        .collect()
}

// TODO: doc
fn parse_ascii_generic(pngii_options: &PngiiOptions, ascii_text: String) -> Vec<Vec<ImageData>> {
    // set up font for rendering
    let font = FontRef::try_from_slice(FONT_BYTES).expect("Could not read input font");

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
        let re = Regex::new(&pattern_string)
            .expect(format!("Error creating regex pattern ({})", pattern).as_str());

        // create the image for this character
        for (full_str, [r, g, b, the_str]) in re.captures_iter(line).map(|c| c.extract()) {
            let red = r.parse::<u8>().expect(
                format!(
                    "Error parsing red from string: ({}), full string: ({}). Improper encoding?",
                    r, full_str
                )
                .as_str(),
            );
            let green = g.parse::<u8>().expect(
                format!(
                    "Error parsing green from string: ({}), full string: ({}). Improper encoding?",
                    g, full_str
                )
                .as_str(),
            );
            let blue = b.parse::<u8>().expect(
                format!(
                    "Error parsing blue from string ({}), full string ({}). Improper encoding?",
                    b, full_str
                )
                .as_str(),
            );

            let generated_png = {
                if the_str.trim().is_empty() {
                    // create a transparent png for a space
                    str_to_transparent_png(pngii_options)
                } else {
                    // render the actual text if it's not empty
                    let colored = ColoredStr {
                        red,
                        green,
                        blue,
                        string: String::from(the_str),
                    };

                    str_to_png(colored, &font, pngii_options)
                        .expect(format!("Could not convert str ({}) to PNG", the_str).as_str())
                }
            };

            char_images.push(generated_png);
        }

        image_2d_vec.push(char_images);
    }

    image_2d_vec
}

/// Reads and converts an image to ASCII and renders it into image.
///
/// # Params
/// * `input_file_name`: The input file name of the image to convert.
/// * `rascii_options`: The `RASCII` image options.
/// * `pngii_options`: The `PNGII` image options.
///
/// # Returns
/// * `Vec<Vec<ImageData>>`: A 2d `Vec` of images, containing each rendered character from the
/// image.
pub fn parse_ascii_to_2d_image_vec(
    input_file_name: &str,
    rascii_options: &RenderOptions,
    pngii_options: &PngiiOptions,
) -> Vec<Vec<ImageData>> {
    let ascii_text = read_image_as_ascii(input_file_name, rascii_options);
    parse_ascii_generic(pngii_options, ascii_text)
}

// TODO: doc
pub fn parse_ascii_to_gif_vec(
    input_file_name: &str,
    rascii_options: &RenderOptions,
    pngii_options: &PngiiOptions,
) -> Vec<(Vec<Vec<ImageData>>, FrameMetadata)> {
    let ascii_text = read_gif_as_ascii(input_file_name, rascii_options);

    // TODO: this can probably cause things to be collected out of order so handle that (probably
    // want to keep an index along with each frame so we can put them back into order at the end)

    // create image data for each frame and keep the frame metadata so we can use it again later
    ascii_text
        .into_par_iter()
        .map(|(image_text, deconstructed_frame)| {
            (
                parse_ascii_generic(pngii_options, image_text),
                deconstructed_frame,
            )
        })
        .collect()
}
