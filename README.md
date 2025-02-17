# ascii_to_png

This is a CPU-only image rendering program that renders colored ANSI-encoded ASCII art and saves them in the
PNG format.

This program is multithreaded to make it faster, but it is very CPU-intensive due to not utilizing the GPU.
Beware of 100% CPU utilization if converting many images in parallel (when converting a batch of images,
which this program can handle for you).

## Usage

```text
Usage: ascii_to_png [OPTIONS] <INPUT_FILENAME> <OUTPUT_FILENAME> [FINAL_IMAGE_INDEX]

Arguments:
  <INPUT_FILENAME>
          Path to the input image
          
          Can also specify a format for an input, if <FINAL_IMAGE_INDEX> is also set to the final input image index.
          
          Example: "input_image%d.png"

  <OUTPUT_FILENAME>
          Path to the output image
          
          Can also specify a format for an input, if <FINAL_IMAGE_INDEX> is also set to the final input image index (will use the same index as the original image).
          
          Example: "output_image%d.png"

  [FINAL_IMAGE_INDEX]
          Allows for converting multiple images. Specifies the final input image index

Options:
  -w, --width <WIDTH>
          Width of the output image. Defaults to 128 if width and height are not specified

  -H, --height <HEIGHT>
          Height of the output image, if not specified, it will be calculated to keep the aspect ratio

  -f, --font-size <FONT_SIZE>
          The font size of the output image. Larger font sizes incur harsher performance penalties.
          
          By default, uses a font size of 16.

  -i, --invert
          Inverts the weights of the characters. Useful for white backgrounds

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

```sh
cargo run -- my_image.png my_ascii.png
```

## Example Output

Original Image:

![original_image](./doc/original_img.png)

Output:

![converted_image](./doc/converted_img.png)
