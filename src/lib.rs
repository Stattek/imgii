pub mod ascii_image_options;
pub mod image_converter;
pub mod image_data;
pub mod image_writer;
pub mod render_char_to_png;
use ascii_image_options::AsciiImageOptions;
use image_converter::parse_ascii;
use image_writer::AsciiImageWriter;
use rascii_art::RenderOptions;

/// The general idea:
/// Use regex to find the rgb values for each character then print each character into its own image
/// Then, from each image that is created, we horizontally merge the character images to form a line of text
/// Finally, from each image containing a line of text, we should vertically merge the images to form a whole image of converted ascii to text.
pub fn convert_ascii_to_png(
    input_file_name: &str,
    output_file_name: &str,
    rascii_options: &RenderOptions,
    ascii_image_options: &AsciiImageOptions,
) {
    let lines = parse_ascii(input_file_name, rascii_options, ascii_image_options);
    let final_image_writer: Option<AsciiImageWriter> =
        AsciiImageWriter::from_2d_vec(lines, ascii_image_options);

    match final_image_writer {
        Some(writer) => {
            writer
                .imagebuf
                .save(&output_file_name)
                .expect(format!("Could not save image {}", output_file_name).as_str());
            println!("Saved PNG {}", output_file_name);
        }
        None => {
            panic!("Could not save the image!");
        }
    }
}
