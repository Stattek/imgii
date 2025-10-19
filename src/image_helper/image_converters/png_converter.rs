use super::generic_converter::render_ascii_generic;
use crate::{
    ImgiiOptions,
    error::{BoxedDynErr, ImgiiError},
    image_helper::image_data::ImageData,
};

use image::open;
use rascii_art::{RenderOptions, render_image_to};

/// Reads and converts an image to ASCII and renders it into image.
///
/// # Params
/// * `input_file_name`: The input file name of the image to convert.
/// * `rascii_options`: The RASCII options for converting to ASCII.
/// * `imgii_options`: The imgii options for rendering ASCII.
///
/// # Returns
/// * `Vec<Vec<ImageData>>`: A 2d `Vec` of images, containing each rendered character from the
///   image.
pub fn parse_ascii_to_2d_png_vec(
    input_file_name: &str,
    rascii_options: &RenderOptions,
    imgii_options: &ImgiiOptions,
) -> Result<Vec<Vec<ImageData>>, ImgiiError> {
    let ascii_text = read_png_as_ascii(input_file_name, rascii_options)?;
    render_ascii_generic(imgii_options, ascii_text)
}

/// Reads the image as an ASCII string using `RASCII`.
///
/// # Params
/// * `input_file_name`: The input file name of the image to convert.
/// * `rascii_options`: The RASCII image options.
///
/// # Returns
/// * `String` containing the colored image data as ASCII, colored using terminal escape sequences.
fn read_png_as_ascii(
    input_file_name: &str,
    rascii_options: &RenderOptions,
) -> Result<String, ImgiiError> {
    // render the ascii text with RASCII
    let mut ascii_text = String::new();
    let loaded_img = open(input_file_name).map_err(|err| -> BoxedDynErr { Box::new(err) })?;
    render_image_to(&loaded_img, &mut ascii_text, rascii_options)
        .map_err(|err| -> BoxedDynErr { Box::new(err) })?;

    Ok(ascii_text)
}
