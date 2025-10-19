pub mod image_helper;
pub mod image_types;
use std::{fs::File, io::BufWriter};

use image::{Frame, codecs::gif::GifEncoder};
use image_helper::{
    ascii_image_options::ImgiiOptions, image_converters::png_converter::parse_ascii_to_2d_png_vec,
    image_writer::AsciiImageWriter,
};
use rascii_art::RenderOptions;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::image_helper::{
    error::{BoxedDynErr, ImgiiError},
    image_converters::gif_converter::read_as_deconstructed_rendered_gif_vec,
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
pub fn convert_to_ascii_png(
    input_file_name: &str,
    output_file_name: &str,
    rascii_options: &RenderOptions,
    imgii_options: &ImgiiOptions,
) -> Result<(), ImgiiError> {
    let lines = parse_ascii_to_2d_png_vec(input_file_name, rascii_options, imgii_options)?;
    let final_image_writer = AsciiImageWriter::new_from_2d_vec(lines, imgii_options)?;

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
pub fn convert_to_ascii_gif(
    input_file_name: &str,
    output_file_name: &str,
    rascii_options: &RenderOptions,
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
                AsciiImageWriter::new_from_2d_vec(image_data, imgii_options),
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
