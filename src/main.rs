mod ascii_image_options;
mod image_converter;
mod image_data;
mod image_writer;
mod render_char_to_png;

use crate::image_converter::parse_ascii;
use crate::image_writer::AsciiImageWriter;
use ascii_image_options::AsciiImageOptions;
use clap::Parser;
use rascii_art::{
    charsets::{self, from_enum, to_charset_enum},
    RenderOptions,
};
use std::{sync::Arc, time::Instant};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to the input image
    ///
    /// Can also specify a format for an input, if <FINAL_IMAGE_INDEX> is also set to the final
    /// input image index.
    ///
    /// Example: "input_image%d.png"
    input_filename: String,

    /// Path to the output image
    ///
    /// Can also specify a format for an input, if <FINAL_IMAGE_INDEX> is also set to the final
    /// input image index (will use the same index as the original image).
    ///
    /// Example: "output_image%d.png"
    output_filename: String,

    /// Width of the output image. Defaults to 128 if width and height are not
    /// specified
    #[arg(short, long)]
    width: Option<u32>,

    /// Height of the output image, if not specified, it will be calculated to
    /// keep the aspect ratio
    #[arg(short = 'H', long)]
    height: Option<u32>,

    /// The font size of the output image.
    /// Larger font sizes incur harsher performance penalties.
    ///
    /// By default, uses a font size of 16.
    #[arg(short, long)]
    font_size: Option<u32>,

    /// Inverts the weights of the characters. Useful for white backgrounds
    #[arg(short, long)]
    invert: bool,

    /// Sets a black background behind the image.
    ///
    /// No background by default.
    #[arg(short, long)]
    background: bool,

    /// Allows for converting multiple images. Specifies the final input image index.
    final_image_index: Option<u32>,

    /// Characters used to render the image, from transparent to opaque.
    /// Built-in charsets: block, emoji, default, russian, slight, minimal
    #[arg(short = 'C', long, default_value = "minimal")]
    charset: String,
}

/// The general idea:
/// Use regex to find the rgb values for each character then print each character into its own image
/// Then, from each image that is created, we horizontally merge the character images to form a line of text
/// Finally, from each image containing a line of text, we should vertically merge the images to form a whole image of converted ascii to text.
fn convert_ascii_to_png(
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

fn main() {
    let pool = threadpool::ThreadPool::new(num_cpus::get());
    let mut args = Args::parse();

    if args.width.is_none() && args.height.is_none() {
        args.width = Some(128);
    }

    let input_name_format = Arc::new(args.input_filename.clone());
    // panic if we don't find the .png extension at the end
    let output_name_format = {
        if !args.output_filename.ends_with(".png") {
            panic!("The <output_filename> argument does not end with the .png extension")
        } else {
            Arc::new(args.output_filename.clone())
        }
    };

    let final_image_index: u32 = args.final_image_index.unwrap_or(1);

    let charset = to_charset_enum(&args.charset).unwrap_or(charsets::Charset::Minimal);

    let rascii_options = Arc::from(RenderOptions {
        width: args.width,
        height: args.height,
        colored: true,
        escape_each_colored_char: true,
        invert: args.invert,
        charset: from_enum(charset),
    });

    let ascii_image_options = Arc::from(AsciiImageOptions::new(args.font_size, args.background));

    let starting_time = Instant::now();
    for i in 1..=final_image_index {
        let input_name_format_arc = Arc::clone(&input_name_format);
        let output_name_format_arc = Arc::clone(&output_name_format);
        let rascii_options_arc = Arc::clone(&rascii_options);
        let ascii_image_options_arc = Arc::clone(&ascii_image_options);

        pool.execute(move || {
            // convert to ascii before performing the conversion
            let input_file_name = input_name_format_arc.replace("%d", i.to_string().as_str());
            let output_file_name = output_name_format_arc.replace("%d", i.to_string().as_str());
            convert_ascii_to_png(
                &input_file_name,
                &output_file_name,
                &rascii_options_arc,
                &ascii_image_options_arc,
            );
        });
    }
    pool.join();
    if pool.panic_count() > 0 {
        eprintln!("---FAIL---");
        eprintln!("{} thread(s) panicked!", pool.panic_count());
    } else {
        println!("---Success!---");
    }
    println!(
        "Time elapsed: {} seconds / {} milliseconds",
        starting_time.elapsed().as_secs(),
        starting_time.elapsed().as_millis()
    );
}
