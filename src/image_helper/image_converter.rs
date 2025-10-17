use std::{fs::File, io::BufReader};

use crate::{
    ImgiiOptions,
    image_helper::{image_data::ImageData, render_char_to_png::str_to_png},
};

use super::render_char_to_png::{ColoredStr, str_to_transparent_png};
use ab_glyph::FontRef;
use image::{AnimationDecoder, Delay, DynamicImage, codecs::gif::GifDecoder, open};
use rascii_art::{RenderOptions, render_image_to};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

// TODO: Read this font at runtime instead and allow the user to choose

// read bytes for the font
const FONT_BYTES: &[u8] = include_bytes!("../../fonts/UbuntuMono.ttf");

/// Holds the metadata for a frame that has been deconstructed.
#[derive(Debug)]
pub struct FrameMetadata {
    /// The left value for this frame.
    pub left: u32,
    /// The top value for this frame.
    pub top: u32,
    /// The delay for this frame.
    pub delay: Delay,
}

impl FrameMetadata {
    /// Creates a new FrameMetadata.
    fn new(left: u32, top: u32, delay: Delay) -> Self {
        Self { left, top, delay }
    }
}

/// Reads a GIF and deconstructs it into an image and its frame metadata for use with converting to
/// ASCII.
///
/// # Params
/// * `input_file_name`: String slice containing the input file name.
///
/// # Returns
/// `Err()` upon error reading the GIF, `Ok()` otherwise.
fn read_deconstructed_gif(
    input_file_name: &str,
) -> Result<Vec<(DynamicImage, FrameMetadata)>, std::io::Error> {
    let file_in = BufReader::new(File::open(input_file_name)?);
    let decoder = match GifDecoder::new(file_in) {
        Ok(decoder) => decoder,
        Err(err) => {
            panic!("Could not read gif {input_file_name} ({err})");
        }
    };

    // decode all of the frames of the gif and then convert each frame into a DynamicImage
    let frames = decoder
        .into_frames()
        .collect_frames()
        .expect(format!("Could not decode gif {}", input_file_name).as_str())
        .into_iter()
        .map(|frame| {
            let left = frame.left();
            let top = frame.top();
            let delay = frame.delay();
            (
                // we split this from the frame metadata because we will not want the original image once we have converted it to ASCII
                frame.into_buffer().into(),
                FrameMetadata::new(left, top, delay),
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

/// Reads a gif as a list of ascii strings, with the frame metadata for the related frame.
///
/// * `input_file_name`: The input file name.
/// * `rascii_options`: The RASCII options for converting to ASCII.
fn read_gif_as_deconstructed_ascii(
    input_file_name: &str,
    rascii_options: &RenderOptions,
) -> Vec<(String, FrameMetadata)> {
    // render the ascii text as images
    let deconstructed_gif = read_deconstructed_gif(input_file_name)
        .expect(format!("Could not read gif {}", input_file_name).as_str());

    // convert the GIF frames to ASCII in parallel
    deconstructed_gif
        .into_par_iter()
        .map(|(image, deconstructed_frame)| {
            let mut ascii_text = String::new();
            // this failing for even a single frame of a GIF is not good
            render_image_to(&image, &mut ascii_text, rascii_options)
                .expect("Could not convert image to ASCII text");
            (ascii_text, deconstructed_frame)
        })
        .collect()
}

/// Generic function for parsing and rendering ASCII into an image.
///
/// * `imgii_options`: The imgii options for rendering ASCII.
/// * `ascii_text`: The ASCII text to render.
fn render_ascii_generic(imgii_options: &ImgiiOptions, ascii_text: String) -> Vec<Vec<ImageData>> {
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
/// * `rascii_options`: The RASCII options for converting to ASCII.
/// * `imgii_options`: The imgii options for rendering ASCII.
///
/// # Returns
/// * `Vec<Vec<ImageData>>`: A 2d `Vec` of images, containing each rendered character from the
/// image.
pub fn parse_ascii_to_2d_image_vec(
    input_file_name: &str,
    rascii_options: &RenderOptions,
    imgii_options: &ImgiiOptions,
) -> Vec<Vec<ImageData>> {
    let ascii_text = read_image_as_ascii(input_file_name, rascii_options);
    render_ascii_generic(imgii_options, ascii_text)
}

/// Reads a GIF and converts it to ASCII. Returns the result containing the image data and frame
/// metadata required to stitch the images back together. The images returned contain the ASCII
/// representation of the original GIF.
///
/// * `input_file_name`: The input file name.
/// * `rascii_options`: The RASCII options for converting to ASCII.
/// * `imgii_options`: The imgii options for rendering ASCII.
///
/// # Returns
/// A vector containing a tuple of (image data, frame metadata) for a particular frame of the
/// resulting GIF.
pub fn read_as_deconstructed_rendered_gif_vec(
    input_file_name: &str,
    rascii_options: &RenderOptions,
    imgii_options: &ImgiiOptions,
) -> Vec<(Vec<Vec<ImageData>>, FrameMetadata)> {
    let ascii_text = read_gif_as_deconstructed_ascii(input_file_name, rascii_options);

    // create image data for each frame and keep the frame metadata so we can use it again later
    ascii_text
        .into_par_iter()
        .map(|(image_text, deconstructed_frame)| {
            (
                render_ascii_generic(imgii_options, image_text),
                deconstructed_frame,
            )
        })
        .collect()
}
