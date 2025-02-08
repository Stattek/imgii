mod image_data;
mod image_reader;
mod image_writer;
mod render_char_to_png;

use crate::image_reader::parse_ascii;
use crate::image_writer::AsciiImageWriter;
use std::{env, process::exit, sync::Arc, time::Instant};

/// The general idea:
/// Use regex to find the rgb values for each character then print each character into its own image
/// Then, from each image that is created, we horizontally merge the character images to form a line of text
/// Finally, from each image containing a line of text, we should vertically merge the images to form a whole image of converted ascii to text.
fn convert_ascii_to_png(input_file_name: &str, output_file_name: &str) {
    let lines = parse_ascii(input_file_name);
    let final_image_writer: Option<AsciiImageWriter> = AsciiImageWriter::from_2d_vec(lines);

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
    let args: Vec<String> = env::args().collect();
    // we will only take 3 or 4 arguments, nothing else
    if args.len() != 3 && args.len() != 4 {
        eprintln!(
            "Usage: ascii_to_png <input_name_format> <output_name_format> <OPTIONAL: final_image_index>\n\t- <input_name_format> can be a format for text files like 'image%d.txt' or it can be a plain input file name if only converting one file.\n\t- <output_name_format> can be a format like 'outimage%d.png' or it can be a plain output file name if only converting one file.\n\t- <final_image_index> is the final name index given to the images that you want to convert (if you want to convert more than one image)."
        );
        exit(1); // fail
    }
    let input_name_format = Arc::new(args[1].clone());
    // panic if we don't find the .png extension at the end
    let output_name_format = {
        if !args[2].ends_with(".png") {
            panic!("The <output_name_format> argument does not end with the .png extension")
        } else {
            Arc::new(args[2].clone())
        }
    };

    let final_image_index: u32 = {
        if args.len() > 3 {
            args[3]
                .parse()
                .expect("Could not convert num_inputs to u32")
        } else {
            1
        }
    };

    let starting_time = Instant::now();
    for i in 1..=final_image_index {
        let copy_input_name_format = Arc::clone(&input_name_format);
        let copy_output_name_format = Arc::clone(&output_name_format);
        pool.execute(move || {
            // convert to ascii before performing the conversion
            let input_file_name = copy_input_name_format.replace("%d", i.to_string().as_str());
            let output_file_name = copy_output_name_format.replace("%d", i.to_string().as_str());
            convert_ascii_to_png(&input_file_name, &output_file_name);
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
