use std::{fs::File, io::BufReader};

use crate::{
    ImgiiOptions,
    image_helper::{
        error::{FileError, ImgiiError},
        image_converters::generic_converter::render_ascii_generic,
        image_data::ImageData,
    },
};

use image::{AnimationDecoder, Delay, DynamicImage, codecs::gif::GifDecoder};
use rascii_art::{RenderOptions, render_image_to};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

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

/// Holds the deconstructed frame data for a single frame, before it is converted to image data.
/// Holds the raw ASCII string and frame metadata.
#[derive(Debug, Clone)]
pub struct NonRenderedFramePart {
    /// The ASCII representation of the frame.
    image_ascii: String,
    /// The frame metadata for this frame.
    frame_metadata: FrameMetadata,
}

/// Holds the deconstructed frame data for a single frame that has been rendered to a 2D vector.
#[derive(Debug, Clone)]
pub struct RenderedFramePart {
    /// The image data with the rendered image data for this frame as a 2D vector.
    image_data: Vec<Vec<ImageData>>,
    /// The frame metadata for this frame.
    frame_metadata: FrameMetadata,
}

/*
* Custom struct impls
*/

impl FrameMetadata {
    /// Creates a new [`FrameMetadata`].
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

impl RenderedFramePart {
    /// Creates a new [`RenderedFramePart`].
    ///
    ///
    /// * `image_data`: The image data.
    /// * `frame_metadata`: The frame metadata.
    #[must_use]
    pub fn new(image_data: Vec<Vec<ImageData>>, frame_metadata: FrameMetadata) -> Self {
        Self {
            image_data,
            frame_metadata,
        }
    }

    /// Gets the image data for this frame.
    #[must_use]
    pub fn image_data(&self) -> &Vec<Vec<ImageData>> {
        &self.image_data
    }

    /// Gets the metadata for this frame.
    #[must_use]
    pub fn frame_metadata(&self) -> &FrameMetadata {
        &self.frame_metadata
    }

    /// Moves out of this RenderedFramePart, returning a tuple containing the image data followed
    /// by metadata.
    #[must_use]
    pub fn into_frame_data(self) -> (Vec<Vec<ImageData>>, FrameMetadata) {
        (self.image_data, self.frame_metadata)
    }
}

impl NonRenderedFramePart {
    /// Creates a new [`NonRenderedFramePart`].
    ///
    ///
    /// * `image_ascii`: The ASCII string representation of an image.
    /// * `frame_metadata`: The frame metadata.
    #[must_use]
    pub fn new(image_ascii: String, frame_metadata: FrameMetadata) -> Self {
        Self {
            image_ascii,
            frame_metadata,
        }
    }
}

/*
* Standalone functions
*/

/// Reads a gif as a list of ascii strings, with the frame metadata for the related frame.
/// Performs a best-effort conversion to ASCII. Some frames may fail to be rendered, which
/// can be handled by the caller.
///
/// * `input_file_name`: The input file name.
/// * `rascii_options`: The RASCII options for converting to ASCII.
fn read_gif_as_deconstructed_ascii(
    input_file_name: &str,
    rascii_options: &RenderOptions,
) -> Result<Vec<Option<NonRenderedFramePart>>, ImgiiError> {
    // render the ascii text as images
    let deconstructed_gif = read_deconstructed_gif(input_file_name)?;

    // convert the GIF frames to ASCII in parallel
    Ok(deconstructed_gif
        .into_par_iter()
        .map(|(image, deconstructed_frame)| {
            let mut ascii_text = String::new();
            // this failing for even a single frame of a GIF is not good, but let's try our best!
            if let Err(_) = render_image_to(&image, &mut ascii_text, rascii_options) {
                None
            } else {
                Some(NonRenderedFramePart::new(ascii_text, deconstructed_frame))
            }
        })
        .collect())
}

/// Reads a gif and converts it to ascii. returns the result containing the image data and frame
/// metadata required to stitch the images back together. the images returned contain the ascii
/// representation of the original gif.
///
/// NOTE: performs a best-effort conversion, some frames may fail and will be returned as a `None`.
///
/// * `input_file_name`: the input file name.
/// * `rascii_options`: the rascii options for converting to ascii.
/// * `imgii_options`: the imgii options for rendering ascii.
pub fn read_as_deconstructed_rendered_gif_vec(
    input_file_name: &str,
    rascii_options: &RenderOptions,
    imgii_options: &ImgiiOptions,
) -> Result<Vec<Option<RenderedFramePart>>, ImgiiError> {
    let ascii_text = read_gif_as_deconstructed_ascii(input_file_name, rascii_options)?;

    // create image data for each frame and keep the frame metadata so we can use it again later
    Ok(ascii_text
        .into_par_iter()
        .filter_map(|frame| frame) // since we can have bad frames, let's just get rid of them
        .map(|frame_part| {
            let rendered_image_res = render_ascii_generic(imgii_options, frame_part.image_ascii);

            match rendered_image_res {
                Ok(rendered_image) => Some(RenderedFramePart::new(
                    rendered_image,
                    frame_part.frame_metadata,
                )),
                Err(err) => {
                    // let's keep trying our best upon error, just give a warning
                    log::warn!("A frame was detected with an error ({err})");
                    None
                }
            }
        })
        .collect())
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
) -> Result<Vec<(DynamicImage, FrameMetadata)>, ImgiiError> {
    let file_in = BufReader::new(
        File::open(input_file_name)
            .map_err(|err| FileError::new(String::from(input_file_name), err.kind()))?,
    );

    // TODO: probably want to make a decode error
    let decoder = match GifDecoder::new(file_in) {
        Ok(decoder) => decoder,
        Err(_) => {
            // the input data in the gif was wrong
            return Err(FileError::new(
                String::from(input_file_name),
                std::io::ErrorKind::InvalidData,
            )
            .into());
        }
    };

    // decode all of the frames of the gif and then convert each frame into a DynamicImage
    let frames = match decoder.into_frames().collect_frames() {
        Ok(frames) => frames,
        Err(_) => {
            // the data is malformed in this GIF
            return Err(FileError::new(
                String::from(input_file_name),
                std::io::ErrorKind::InvalidData,
            )
            .into());
        }
    };
    let ret = frames
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

    Ok(ret)
}
