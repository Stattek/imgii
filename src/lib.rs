//! Imgii is a library for converting images to ASCII and rendering as different image types. For
//! example, it can take a PNG input and convert it into ASCII, render it, and save it.

pub mod conversion;
pub mod error;
pub mod image_types;
pub mod options;

use std::{fs::File, io::BufWriter};

use image::{Frame, codecs::gif::GifEncoder};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    conversion::{
        converters::{
            gif_converter::read_as_deconstructed_rendered_gif_vec,
            png_converter::parse_ascii_to_2d_png_vec,
        },
        image_writer::AsciiImageWriter,
    },
    error::{BoxedDynErr, ImgiiError},
    options::{ImgiiOptions, RasciiOptions},
};

/// Converts an image (such as a PNG or JPEG) into an ASCII PNG.
/// It does this by first converting the image into colored ASCII text,
/// then renders the ASCII text as an image.
///
/// # Params
/// - `input_file_name` - The input file name.
/// - `output_file_name` - The output file name.
/// - `rascii_options` - The `RASCII` render options.
/// - `imgii_options` - The `imgii` render options
///
/// # Returns
/// - `Err(())` upon error, `Ok(())` otherwise.
///
/// # Example
///
/// ```
/// // Simple example showing off basic usage.
///
/// use imgii::{
///     convert_to_ascii_png,
///     options::{Charset, ImgiiOptionsBuilder, RasciiOptions, from_enum},
/// };
///
/// let input_file_name = "the_input_image.jpg";
/// let output_file_name = "the_output_image.png";
///
/// // rascii options (for converting image to ASCII text)
/// let rascii_options = RasciiOptions::new()
///     .colored(true)
///     .escape_each_colored_char(true)
///     .charset(from_enum(Charset::Minimal));
///
/// // imgii options (for converting image to ASCII image)
/// let imgii_options = ImgiiOptionsBuilder::new().font_size(16).background(false).build();
///
/// // perform the conversion
/// match convert_to_ascii_png(
///     &input_file_name,
///     &output_file_name,
///     &rascii_options,
///     &imgii_options,
/// ) {
///     Ok(_) => {
///         // success!
///         println!("Saved PNG {}", output_file_name);
///     }
///     Err(err) => {
///         // failure
///         panic!("Could not save PNG {} ({})", output_file_name, err);
///     }
/// };
/// ```
pub fn convert_to_ascii_png(
    input_file_name: &str,
    output_file_name: &str,
    rascii_options: &RasciiOptions,
    imgii_options: &ImgiiOptions,
) -> Result<(), ImgiiError> {
    let lines = parse_ascii_to_2d_png_vec(input_file_name, rascii_options, imgii_options)?;
    let final_image_writer = AsciiImageWriter::from_2d_vec(lines, imgii_options)?;

    // write the image
    final_image_writer
        .imagebuf
        .as_buffer()
        .save(&output_file_name)
        .map_err(|err| -> BoxedDynErr { Box::new(err) })?;
    Ok(())
}

/// Converts a GIF into an ASCII GIF.
/// It does this by first converting the image into colored ASCII text,
/// then renders the ASCII text as an image.
///
/// # Params
/// - `input_file_name` - The input file name.
/// - `output_file_name` - The output file name.
/// - `rascii_options` - The `RASCII` render options.
/// - `imgii_options` - The `imgii` render options
///
/// # Returns
/// - `Err(())` upon error, `Ok(())` otherwise.
///
/// # Example
///
/// ```
/// // Simple example showing off basic usage.
///
/// use imgii::{
///     convert_to_ascii_gif,
///     options::{Charset, ImgiiOptionsBuilder, RasciiOptions, from_enum},
/// };
///
/// let input_file_name = "the_input_image.gif";
/// let output_file_name = "the_output_image.gif";
///
/// // rascii options (for converting image to ASCII text)
/// let rascii_options = RasciiOptions::new()
///     .colored(true)
///     .escape_each_colored_char(true)
///     .charset(from_enum(Charset::Minimal));
///
/// // imgii options (for converting image to ASCII image)
/// let imgii_options = ImgiiOptionsBuilder::new().build();
///
/// // perform the conversion
/// match convert_to_ascii_gif(
///     &input_file_name,
///     &output_file_name,
///     &rascii_options,
///     &imgii_options,
/// ) {
///     Ok(_) => {
///         println!("Saved GIF {}", output_file_name);
///     }
///     Err(err) => {
///         panic!("Could not save GIF {} ({})", output_file_name, err);
///     }
/// };
/// ```
pub fn convert_to_ascii_gif(
    input_file_name: &str,
    output_file_name: &str,
    rascii_options: &RasciiOptions,
    imgii_options: &ImgiiOptions,
) -> Result<(), ImgiiError> {
    let raw_frames =
        read_as_deconstructed_rendered_gif_vec(input_file_name, rascii_options, imgii_options)?;

    // create an image writer for each frame
    let image_writers = raw_frames
        .into_par_iter()
        // filter out failed frames
        .filter_map(|frame_part| frame_part)
        .map(|frame_part| {
            let (image_data, frame_metadata) = frame_part.into_frame_data();
            (
                AsciiImageWriter::from_2d_vec(image_data, imgii_options),
                frame_metadata,
            )
        })
        .collect::<Vec<_>>();

    let frames: Vec<Frame> = image_writers
        .into_par_iter()
        .filter_map(|(writer, frame_metadata)| match writer {
            // let's just get rid of errors and try our best with what we've got
            Ok(writer) => Some((writer, frame_metadata)),
            Err(_) => None,
        })
        .map(|(image_writer, frame_metadata)| {
            // basically, we want to put the image data and the frame data back into a frame, so we
            // can then use the image crate to build a new GIF from the new image!
            Frame::from_parts(
                image_writer.imagebuf.into(), // converts into its inner held type
                frame_metadata.left(),
                frame_metadata.top(),
                frame_metadata.delay(),
            )
        })
        .collect();

    let out_file = File::create(output_file_name)?;
    let file_writer = BufWriter::new(out_file);

    let mut gif_encoder = GifEncoder::new(file_writer);

    // TODO: allow user to choose number of repeats?
    let err = gif_encoder.set_repeat(image::codecs::gif::Repeat::Infinite);
    if let Err(err) = err {
        // repeat couldn't be set properly
        let err_box: BoxedDynErr = Box::new(err);
        return Err(err_box.into());
    }

    // FUTURE: the longest part of the GIF creation process is encoding...is there any way to speed
    // it up?

    // encode the frames
    match gif_encoder.encode_frames(frames) {
        Err(err) => {
            let err_box: BoxedDynErr = Box::new(err);
            Err(err_box.into())
        }
        _ => Ok(()),
    }
}
