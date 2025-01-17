mod image_data;
mod image_writer;
mod render_char_to_png;

use image_writer::MyImageWriter;
use render_char_to_png::{str_to_png, str_to_transparent_png, ColoredStr};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
    sync::Arc,
    time::Instant,
};

use regex::Regex;

/// The general idea:
/// Use regex to find the rgb values for each character then print each character into its own image
/// Then, from each image that is created, we horizontally merge the character images to form a line of text
/// Finally, from each image containing a line of text, we should vertically merge the images to form a whole image of converted ascii to text.
fn convert_ascii_to_png(input_file_name: &str, output_file_name: &str) {
    let infile = File::open(&input_file_name)
        .expect(format!("Error opening input file {}", input_file_name).as_str());
    let reader = BufReader::new(infile);

    // contains lines of images
    // starting at 0 is the top, first line of the vector
    // inside an inner vec, 0 starts at the leftmost character of the line
    let mut lines = vec![];
    // read every line in the file
    for line in reader.lines().flatten() {
        let mut char_images = vec![];
        // we need to find each character that we are going to write
        // we assume that there's only one character for each color
        let pattern = r"\[38;2;([0-9]+);([0-9]+);([0-9]+)m(.)";
        let _control_char = '\u{1b}'; // another way to represent the ansi escape character `\033`
        let re = Regex::new(pattern)
            .expect(format!("Error creating regex pattern ({})", pattern).as_str());
        // create the image for this character
        for (full_str, [r, g, b, the_str]) in re.captures_iter(&line).map(|c| c.extract()) {
            let red = r.parse::<u8>().expect(
                format!(
                    "Error parsing red from string: ({}), full string: ({}). Improper encoding?",
                    r, full_str
                )
                .as_str(),
            );
            let green = g.parse::<u8>().expect(
                format!(
                    "Error parsing green from string: ({}), full string: ({}). Improper encoding?",
                    g, full_str
                )
                .as_str(),
            );
            let blue = b.parse::<u8>().expect(
                format!(
                    "Error parsing blue from string ({}), full string ({}). Improper encoding?",
                    b, full_str
                )
                .as_str(),
            );

            let generated_png = {
                if the_str.trim().is_empty() {
                    // create a transparent png for a space
                    str_to_transparent_png()
                } else {
                    // render the actual text if it's not empty
                    let colored = ColoredStr {
                        red,
                        green,
                        blue,
                        string: String::from(the_str),
                    };

                    str_to_png(colored)
                        .expect(format!("Could not convert str ({}) to PNG", the_str).as_str())
                }
            };

            char_images.push(generated_png);
        }

        lines.push(char_images);
    }

    let mut image_writers = vec![];
    for line in lines {
        let image_writer = {
            if line.len() >= 2 {
                // if the current line is at least two images, then we append them together and then the rest of them
                let mut the_writer = MyImageWriter::new_append_right(&line[0], &line[1]);

                for i in 2..line.len() {
                    // append all of the images in this line
                    the_writer.append_right(&line[i]);
                }

                the_writer
            } else if line.len() > 0 {
                MyImageWriter::from_imagedata(line[0].clone())
            } else {
                // we don't have anything to write
                eprintln!("WARNING: Skipped an empty line of images");
                continue;
            }
        };
        image_writers.push(image_writer);
    }

    let final_image_writer: Option<MyImageWriter> = {
        if image_writers.len() >= 2 {
            // if the current line is at least two images, then we append them together and then the rest of them
            let mut the_writer = MyImageWriter::new_append_down(
                &image_writers[0].imagebuf,
                &image_writers[1].imagebuf,
            );

            for i in 2..image_writers.len() {
                // append all of the images in this line
                the_writer.append_down(&image_writers[i].imagebuf);
            }

            Some(the_writer)
        } else if image_writers.len() > 0 {
            Some(MyImageWriter::from_imagedata(
                image_writers[0].imagebuf.clone(),
            ))
        } else {
            // we don't have anything to write
            None
        }
    };

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
    println!("---Success!---");
    println!(
        "Time elapsed: {} seconds / {} milliseconds",
        starting_time.elapsed().as_secs(),
        starting_time.elapsed().as_millis()
    );
}
