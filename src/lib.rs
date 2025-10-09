#![allow(non_snake_case)]
pub mod image_helper;
pub mod image_types;
use std::{fs::File, io::BufWriter};

use image::{Frame, codecs::gif::GifEncoder};
use image_helper::{
    ascii_image_options::PngiiOptions, image_converter::parse_ascii_to_2d_image_vec,
    image_writer::AsciiImageWriter,
};
use rascii_art::RenderOptions;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::image_helper::image_converter::{FrameMetadata, parse_ascii_to_gif_vec};

/// Converts an image (such as a PNG or JPEG) into an ASCII PNG.
/// It does this by first converting the image into colored ASCII art,
/// then renders the ASCII art as an image.
///
/// # Params
/// - `input_file_name` - The input file name.
/// - `output_file_name` - The output file name.
/// - `rascii_options` - The `RASCII` render options.
/// - `pngii_options` - The `PNGII` render options
///
/// # Returns
/// - `Err(())` upon error, `Ok(())` otherwise.
pub fn convert_to_ascii_png(
    input_file_name: &str,
    output_file_name: &str,
    rascii_options: &RenderOptions,
    pngii_options: &PngiiOptions,
) -> Result<(), ()> {
    let lines = parse_ascii_to_2d_image_vec(input_file_name, rascii_options, pngii_options);
    let final_image_writer: Option<AsciiImageWriter> =
        AsciiImageWriter::from_2d_vec(lines, pngii_options);

    match final_image_writer {
        Some(writer) => {
            match writer.imagebuf.save(&output_file_name) {
                Ok(_) => {
                    // do nothing, image saved properly
                }
                Err(_) => {
                    // return error, the image could not be saved
                    return Err(());
                }
            }
            Ok(())
        }
        None => Err(()),
    }
}

/// Converts a GIF into an ASCII GIF.
/// It does this by first converting the iamge into colored ASCII art,
/// then renders the ASCII art as an image.
///
/// # Params
/// - `input_file_name` - The input file name.
/// - `output_file_name` - The output file name.
/// - `rascii_options` - The `RASCII` render options.
/// - `pngii_options` - The `PNGII` render options
///
/// # Returns
/// - `Err(())` upon error, `Ok(())` otherwise.
pub fn convert_to_ascii_gif(
    input_file_name: &str,
    output_file_name: &str,
    rascii_options: &RenderOptions,
    pngii_options: &PngiiOptions,
) -> Result<(), ()> {
    let raw_frames = parse_ascii_to_gif_vec(input_file_name, rascii_options, pngii_options);

    // FIXME: again, we might be breaking the order of all these images

    // create an image writer for each frame
    let image_writers: Vec<(Option<AsciiImageWriter>, FrameMetadata)> = raw_frames
        .into_par_iter()
        .map(|(image_data, frame_metadata)| {
            (
                AsciiImageWriter::from_2d_vec(image_data, pngii_options),
                frame_metadata,
            )
        })
        .collect();

    // DEBUG: now we need to bring the gif together frame-by-frame. Check that this is accurate.
    let frames: Vec<Frame> = image_writers
        .into_par_iter()
        .map(|(image_writer, frame_metadata)| match image_writer {
            // basically, we want to put the image data and the frame data back into a frame, so we
            // can then use the image crate to build a new GIF from the new image!
            Some(image_writer) => Some(Frame::from_parts(
                image_writer.imagebuf.into(), // converts into its inner held type
                frame_metadata.left,
                frame_metadata.top,
                frame_metadata.delay,
            )),
            None => None,
        })
        .filter_map(|frame| match frame {
            Some(frame) => Some(frame),
            None => None,
        })
        .collect();

    let file_writer = BufWriter::new(
        File::create(output_file_name)
            .expect(format!("The file {output_file_name} already exists!").as_str()),
    );
    let mut gif_encoder = GifEncoder::new(file_writer);
    // encode the frames
    match gif_encoder.encode_frames(frames) {
        Err(err) => {
            // TODO: what does this print even do?
            eprintln!(
                "Could encode frames for image {} ({})",
                output_file_name, err
            );
            Err(())
        }
        _ => Ok(()),
    }
}
