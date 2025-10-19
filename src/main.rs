use clap::Parser;
use clap::builder as clap_builder;
use clap::builder::styling as clap_styling;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{sync::Arc, time::Instant};

use imgii::{
    convert_to_ascii_gif, convert_to_ascii_png,
    image_types::{IMG_TYPES_ARRAY, ImageBatchType, OutputImageType},
    options::{
        Charset, ImgiiOptions, ImgiiOptionsBuilder, RasciiOptions, convert_string_to_str_vec,
        from_enum, to_charset_enum,
    },
};

#[derive(Debug, Parser)]
#[command(author, version, about, styles=set_color_style())]
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

    /// Width (in characters) of the output image. To retain the image's original aspect ratio,
    /// only set this value.
    ///
    /// Defaults to 128 if width and height are not specified.
    #[arg(short, long)]
    width: Option<u32>,

    /// Height (in characters) of the output image, if not specified, it will be calculated to keep
    /// the aspect ratio
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
    /// Index starts at 1.
    final_image_index: Option<u32>,

    /// Characters used to render the image, from transparent to opaque.
    /// Built-in charsets: [block, emoji, default, russian, slight, minimal]
    #[arg(short = 'C', long, default_value = "minimal")]
    charset: String,

    /// Character override. Ignores the current charset and repeats the desired string for the
    /// entirety of the output image.
    #[arg(short = 'o', long)]
    char_override: Option<String>,
}

// default values for arguments
const DEFAULT_WIDTH: u32 = 128;

/// Sets the style for clap output.
fn set_color_style() -> clap_builder::Styles {
    clap_builder::Styles::styled()
        .usage(
            clap_styling::Style::new().fg_color(Some(clap_styling::Color::Ansi(
                clap_styling::AnsiColor::Yellow,
            ))),
        )
        .header(
            clap_styling::Style::new().fg_color(Some(clap_styling::Color::Ansi(
                clap_styling::AnsiColor::BrightMagenta,
            ))),
        )
        .literal(
            clap_styling::Style::new().fg_color(Some(clap_styling::Color::Ansi(
                clap_styling::AnsiColor::Magenta,
            ))),
        )
        .error(
            clap_styling::Style::new().fg_color(Some(clap_styling::Color::Ansi(
                clap_styling::AnsiColor::BrightRed,
            ))),
        )
        .context_value(
            clap_styling::Style::new().fg_color(Some(clap_styling::Color::Ansi(
                clap_styling::AnsiColor::BrightCyan,
            ))),
        )
        .context(
            clap_styling::Style::new().fg_color(Some(clap_styling::Color::Ansi(
                clap_styling::AnsiColor::Green,
            ))),
        )
        .invalid(
            clap_styling::Style::new().fg_color(Some(clap_styling::Color::Ansi(
                clap_styling::AnsiColor::Red,
            ))),
        )
        .placeholder(
            clap_styling::Style::new().fg_color(Some(clap_styling::Color::Ansi(
                clap_styling::AnsiColor::Blue,
            ))),
        )
        .valid(
            clap_styling::Style::new().fg_color(Some(clap_styling::Color::Ansi(
                clap_styling::AnsiColor::BrightGreen,
            ))),
        )
}

/// Sets up threads for this program.
/// NOTE: Must be run only once.
#[inline(always)]
fn setup_threads() {
    let the_num_cpus = num_cpus::get();
    let err = rayon::ThreadPoolBuilder::new()
        .num_threads(the_num_cpus)
        .build_global();
    if let Err(err) = err {
        panic!(
            "Could not create a thread pool for program. Has it been created already? Num threads = {the_num_cpus}. ({err})"
        );
    }
}

/// Creates an instance of [`ImgiiOptions`] for the CLI for imgii.
///
/// * `font_size`: The font size argument.
/// * `background`: The background flag.
fn create_imgii_options(font_size: Option<u32>, background: bool) -> ImgiiOptions {
    let mut builder = ImgiiOptionsBuilder::new().background(background);

    // set values that might not exist. The builder will choose its own defaults if not specified
    if let Some(font_size) = font_size {
        builder = builder.font_size(font_size);
    }

    builder.build()
}

/// Creates [`RasciiOptions`] for RASCII.
///
/// * `width`: The width.
/// * `height`: The height.
/// * `invert`: Whether image should be inverted.
/// * `rascii_charset`: The RASCII charset enum value.
/// * `char_override`: The char override value.
fn create_rascii_options<'a>(
    width: Option<u32>,
    height: Option<u32>,
    invert: bool,
    rascii_charset: Charset,
    char_override: Option<String>,
) -> RasciiOptions<'a> {
    let mut builder = RasciiOptions::new();

    // build the complex values first
    if let Some(width) = width {
        builder = builder.width(width);
    }
    if let Some(height) = height {
        builder = builder.height(height);
    }
    if let Some(char_override) = char_override {
        // converts the string to a string vec if it is Some, otherwise stores as None
        builder = builder.char_override(convert_string_to_str_vec(char_override));
    }

    builder
        .colored(true)
        .escape_each_colored_char(true)
        .invert(invert)
        .charset(from_enum(rascii_charset))
}

fn main() {
    let mut args = Args::parse();
    env_logger::init();
    setup_threads();

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

    let rascii_charset = to_charset_enum(&args.charset).unwrap_or(Charset::Minimal);

    // the options for RASCII for converting to ASCII under the hood
    let rascii_options = create_rascii_options(
        args.width,
        args.height,
        args.invert,
        rascii_charset,
        args.char_override,
    );
    log::debug!("RASCII options = {:?}", rascii_options);

    // are we doing a batch of images or a single image
    let batch_type = if let Some(final_image_idx) = args.final_image_index {
        ImageBatchType::Batch {
            final_index: final_image_idx,
        }
    } else {
        ImageBatchType::Single
    };

    // our options for rendering ASCII in imgii
    let imgii_options = create_imgii_options(args.font_size, args.background);
    log::debug!("imgii options = {:?}", imgii_options);

    // Now, handle the conversion
    match image_type {
        OutputImageType::Png => {
            match batch_type {
                ImageBatchType::Batch {
                    final_index: final_image_idx,
                } => {
                    log::debug!("Converting batch of PNGs...");
                    // handle converting a batch of images
                    convert_png_batch(
                        final_image_idx,
                        Arc::from(input_name_format),
                        Arc::from(output_name_format),
                        Arc::from(rascii_options),
                        Arc::from(imgii_options),
                    );
                }
                ImageBatchType::Single => {
                    log::debug!("Converting single PNG...");
                    match convert_to_ascii_png(
                        &input_name_format,
                        &output_name_format,
                        &rascii_options,
                        &imgii_options,
                    ) {
                        Ok(_) => {}
                        Err(_) => {
                            log::error!("Could not save PNG {}", output_name_format);
                        }
                    };
                }
            };
        }
        OutputImageType::Gif => {
            match batch_type {
                ImageBatchType::Batch {
                    final_index: final_img_idx,
                } => {
                    // this line was really long, but with a little magic, we can shorten it
                    panic!(
                        "Cannot convert a batch of GIFs, argument final_img_idx={final_img_idx}. {}",
                        "Do not set this argument if intending to convert a GIF."
                    );
                }
                ImageBatchType::Single => {
                    log::debug!("Converting single GIF");
                    match convert_to_ascii_gif(
                        &input_name_format,
                        &output_name_format,
                        &rascii_options,
                        &imgii_options,
                    ) {
                        Ok(_) => {
                            log::info!("Saved GIF {}", output_name_format);
                        }
                        Err(err) => {
                            log::error!("Could not save GIF {} ({})", output_name_format, err);
                        }
                    }
                }
            };
        }
    }
}

/// Renders a batch of PNGs as ASCII and saves to PNG.
///
/// * `final_image_index`: The final image index of input PNGs.
/// * `input_name_format`: The input name format for input PNGs.
/// * `output_name_format`: The output name format for saved PNGs.
/// * `rascii_options`: The RASCII options for generating ASCII text.
/// * `imgii_options`: The imgii options for rendering ASCII as PNG.
///
/// # Panics
/// If a thread fails to convert an image to ASCII, this will cause the program to panic.
fn convert_png_batch(
    final_image_index: u32,
    input_name_format: Arc<String>,
    output_name_format: Arc<String>,
    rascii_options: Arc<RasciiOptions<'static>>,
    imgii_options: Arc<ImgiiOptions>,
) {
    let starting_time = Instant::now();

    // NOTE: if a single thread panics here, the whole program panics
    (1..=final_image_index).into_par_iter().for_each(|i| {
        let input_name_format_arc = Arc::clone(&input_name_format);
        let output_name_format_arc = Arc::clone(&output_name_format);
        let rascii_options_arc = Arc::clone(&rascii_options);
        let imgii_options_arc = Arc::clone(&imgii_options);

        // convert to ascii before performing the conversion
        let input_file_name = input_name_format_arc.replace("%d", i.to_string().as_str());
        let output_file_name = output_name_format_arc.replace("%d", i.to_string().as_str());
        match convert_to_ascii_png(
            &input_file_name,
            &output_file_name,
            &rascii_options_arc,
            &imgii_options_arc,
        ) {
            Ok(_) => {
                log::info!("Saved PNG {}", output_file_name);
            }
            Err(err) => {
                panic!("Could not save PNG {} ({})", output_file_name, err);
            }
        };
    });

    log::info!("---Success!---");
    log::info!(
        "Time elapsed: {} seconds / {} milliseconds",
        starting_time.elapsed().as_secs(),
        starting_time.elapsed().as_millis()
    );
}
