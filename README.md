# PNGII

PNGII is an image rendering program that renders colored ANSI-encoded ASCII art as a PNG and saves the output file.
Supports popular image types, such as PNG, JPEG, GIF, WEBP, and more.

## Building

```sh
git clone https://github.com/Stattek/PNGII.git
cd PNGII
git submodule update --init --recursive
cargo build
```

## Usage

```text
Usage: PNGII [OPTIONS] <INPUT_FILENAME> <OUTPUT_FILENAME> [FINAL_IMAGE_INDEX]

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

  -b, --background
          Sets a black background behind the image.

          No background by default.

  -C, --charset <CHARSET>
          Characters used to render the image, from transparent to opaque. Built-in charsets: block, emoji, default, russian, slight, minimal

          [default: minimal]

  -o, --char-override <CHAR_OVERRIDE>
          Character override. Ignores the current charset and repeats the desired string for the entirety of the output image

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

```sh
cargo run --release -- my_image.png my_ascii.png
```

### Example Usage With Args

```sh
# renders an image with ASCII characters with a width of 150, using the block charset
cargo run --release -- --charset block --width 150 my_image.png my_ascii.png
```

## Example Output

Original Image:

![original_image](./doc/original_img.png)

Output:

![converted_image](./doc/converted_img.png)
