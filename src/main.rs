use clap::Parser;
use pngii::image_helper::ascii_image_options::PngiiOptions;
use pngii::image_types::{IMG_TYPES_ARRAY, ImageBatchType};
use pngii::{convert_to_ascii_gif, image_types};
use pngii::{convert_to_ascii_png, image_types::OutputImageType};
use rascii_art::{
    RenderOptions,
    charsets::{self, from_enum, to_charset_enum},
    convert_string_to_str_vec,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
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

    /// Character override. Ignores the current charset and repeats the desired string for the
    /// entirety of the output image.
    #[arg(short = 'o', long)]
    char_override: Option<String>,
}

// default values for arguments
const DEFAULT_WIDTH: u32 = 128;

fn main() {
    let mut args = Args::parse();
    env_logger::init();

    if args.width.is_none() && args.height.is_none() {
        args.width = Some(DEFAULT_WIDTH);
    }

    let input_name_format = args.input_filename.clone();
    let output_name_format = args.output_filename.clone();

    // see what image type we are working with and panic if it's unrecognized
    let image_type = match OutputImageType::from_file_name(&args.output_filename) {
        Some(image_type) => image_type,
        None => {
            panic!(
                "Could not get output file type from {}, expected one of ({})",
                args.output_filename,
                IMG_TYPES_ARRAY.join(", ")
            );
        }
    };

    let rascii_charset = to_charset_enum(&args.charset).unwrap_or(charsets::Charset::Minimal);

    // the options for RASCII for converting to ASCII under the hood
    let rascii_options = RenderOptions {
        width: args.width,
        height: args.height,
        colored: true,
        escape_each_colored_char: true,
        invert: args.invert,
        charset: from_enum(rascii_charset),
        char_override: if let Some(char_override) = args.char_override {
            Some(convert_string_to_str_vec(char_override))
        } else {
            None
        },
    };

    // are we doing a batch of images or a single image
    let batch_type = if let Some(final_image_idx) = args.final_image_index {
        ImageBatchType::BatchWithFinalIdx(final_image_idx)
    } else {
        ImageBatchType::Single
    };

    // our options for rendering ASCII
    let pngii_options = PngiiOptions::new(args.font_size, args.background);

    // Now, handle the conversion
    match image_type {
        OutputImageType::Png => {
            match batch_type {
                ImageBatchType::BatchWithFinalIdx(final_image_idx) => {
                    // handle converting a batch of images
                    convert_png_batch(
                        final_image_idx,
                        Arc::from(input_name_format),
                        Arc::from(output_name_format),
                        Arc::from(rascii_options),
                        Arc::from(pngii_options),
                    );
                }
                ImageBatchType::Single => {
                    match convert_to_ascii_png(
                        &input_name_format,
                        &output_name_format,
                        &rascii_options,
                        &pngii_options,
                    ) {
                        Ok(_) => {
                            println!("Saved PNG {}", output_name_format);
                        }
                        Err(_) => {
                            eprintln!("Could not save PNG {}", output_name_format);
                        }
                    };
                }
            };
        }
        OutputImageType::Gif => {
            match batch_type {
                ImageBatchType::BatchWithFinalIdx(final_img_idx) => {
                    panic!(
                        "Cannot convert a batch of GIFs, argument final_img_idx={final_img_idx}"
                    );
                }
                ImageBatchType::Single => {
                    match convert_to_ascii_gif(
                        &input_name_format,
                        &output_name_format,
                        &rascii_options,
                        &pngii_options,
                    ) {
                        Ok(_) => {
                            println!("Saved GIF {}", output_name_format);
                        }
                        Err(_) => {
                            eprintln!("Could not save GIF {}", output_name_format);
                        }
                    }
                }
            };
        }
    }
}

fn convert_png_batch(
    final_image_index: u32,
    input_name_format: Arc<String>,
    output_name_format: Arc<String>,
    rascii_options: Arc<RenderOptions<'static>>,
    pngii_options: Arc<PngiiOptions>,
) {
    let starting_time = Instant::now();
    // TODO: check what happens if we get a panic in a thread
    (1..=final_image_index).into_par_iter().for_each(|i| {
        let input_name_format_arc = Arc::clone(&input_name_format);
        let output_name_format_arc = Arc::clone(&output_name_format);
        let rascii_options_arc = Arc::clone(&rascii_options);
        let pngii_options_arc = Arc::clone(&pngii_options);

        // convert to ascii before performing the conversion
        let input_file_name = input_name_format_arc.replace("%d", i.to_string().as_str());
        let output_file_name = output_name_format_arc.replace("%d", i.to_string().as_str());
        match convert_to_ascii_png(
            &input_file_name,
            &output_file_name,
            &rascii_options_arc,
            &pngii_options_arc,
        ) {
            Ok(_) => {
                println!("Saved PNG {}", output_file_name);
            }
            Err(_) => {
                // TODO: check this
                panic!("Could not save PNG {}", output_file_name);
            }
        };
    });

    println!("---Success!---");
    println!(
        "Time elapsed: {} seconds / {} milliseconds",
        starting_time.elapsed().as_secs(),
        starting_time.elapsed().as_millis()
    );
}
