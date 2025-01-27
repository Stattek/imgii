# ascii_to_png

This is a CPU-only image rendering program that renders colored ANSI-encoded ASCII art and saves them in the
PNG format.

This program DOES NOT work with pure, non-colored ASCII art.

To attain ascii with this coloring, some ascii programs like `jp2a`
can create ascii text files that work with this program, as they create ANSI color escape sequences
for one character at a time (as of creating this program, `jp2a` version 1.1.1 has this behavior
**and is the version tested when creating this program**).

This program is multithreaded to make it faster, but it is very CPU-intensive due to not utilizing the GPU.
Beware of 100% CPU utilization if converting many images in parallel (when converting a batch of images,
which this program can handle for you).

## Usage

Using jp2a, we can convert an image `my_image.png` into colored ANSI-enconded
ASCII art with the following command, saving the output in `my_ascii.txt`.

```sh
# you can set whichever width you want, depending on what looks best. This is if we wanted a width of 80.
jp2a --colored --width=80 my_image.png > my_ascii.txt
```

Then, we can run the program on that file and convert it to a PNG, saving it as
`my_ascii.png`.

```sh
cargo run -- my_ascii.txt my_ascii.png
```

## Example Output

Original Image:
![107233826](https://github.com/user-attachments/assets/0ac74859-78d2-41d7-96a2-16390ba5d1ec)

Output:
![image](https://github.com/user-attachments/assets/667ff6bb-3152-4d18-aa87-dac9aa6b179a)

## Future

In the future, it would be ideal for this program to also handle converting the image into the ASCII
format required for running this program. This would simplify the process of conversion for users and
would remove the pain of having to use external programs to use this program.
