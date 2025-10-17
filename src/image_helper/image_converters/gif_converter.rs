use std::{fs::File, io::BufReader};

use crate::{ImgiiOptions, image_helper::image_data::ImageData};

use super::generic_converter::render_ascii_generic;
use image::{AnimationDecoder, Delay, DynamicImage, codecs::gif::GifDecoder};
use rascii_art::{RenderOptions, render_image_to};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// Holds the metadata for a frame that has been deconstructed.
#[derive(Debug, Clone)]
pub struct FrameMetadata {
    /// The left value for this frame.
    left: u32,
    /// The top value for this frame.
    top: u32,
    /// The delay for this frame.
    delay: Delay,
}

impl FrameMetadata {
    /// Creates a new FrameMetadata.
    #[must_use]
    pub fn new(left: u32, top: u32, delay: Delay) -> Self {
        Self { left, top, delay }
    }

    /// Gets the x offset for this frame.
    #[must_use]
    pub fn left(&self) -> u32 {
        self.left
    }

    /// Gets the y offset for this frame.
    #[must_use]
    pub fn top(&self) -> u32 {
        self.top
    }

    /// Gets the delay of this frame.
    #[must_use]
    pub fn delay(&self) -> Delay {
        self.delay
    }
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
        .unwrap_or_else(|_| panic!("Could not read gif {}", input_file_name));

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
        .unwrap_or_else(|_| panic!("Could not decode gif {}", input_file_name))
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
