#![allow(non_snake_case)]
pub mod image_helper;
pub mod image_types;
use image_helper::{
    ascii_image_options::AsciiImageOptions, image_converter::parse_ascii_to_2d_image_vec,
    image_writer::AsciiImageWriter,
};
use rascii_art::RenderOptions;

/// Converts an image (such as a PNG or JPEG) into an ASCII PNG.
/// It does this by first converting the iamge into colored ASCII art,
/// then renders the ASCII art as an image.
///
/// # Params
/// - `input_file_name` - The input file name.
/// - `output_file_name` - The output file name.
/// - `rascii_options` - The `RASCII` render options.
/// - `ascii_image_options` - The `PNGII` render options
///
/// # Returns
/// - `Err(())` upon error, `Ok(())` otherwise.
pub fn convert_image_to_ascii_png(
    input_file_name: &str,
    output_file_name: &str,
    rascii_options: &RenderOptions,
    ascii_image_options: &AsciiImageOptions,
) -> Result<(), ()> {
    let lines = parse_ascii_to_2d_image_vec(input_file_name, rascii_options, ascii_image_options);
    let final_image_writer: Option<AsciiImageWriter> =
        AsciiImageWriter::from_2d_vec(lines, ascii_image_options);

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
