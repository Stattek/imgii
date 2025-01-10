# ascii_to_png
- This is a not well optimized program that renders ascii text with ANSI color escape sequences that contain only one letter per each escape sequence. I made it for fun.
- This program does not work with non-colored ascii text.
- To attain ascii with this coloring, some ascii programs like `jp2a` can create ascii text files that work with this program, as they create ANSI color escape sequences for one character at a time (as of creating this program, `jp2a` version 1.1.1 has this behavior).
- This program is multithreaded to make it faster, but it is very expensive. Beware of 100% CPU utilization.

# TODO:
- Optimize the program, as it does a lot of copying of images to get to the fully-created final ASCII png. This could be made better possibly by creating one ImageBuffer to hold the final image, and taking each individual ImageBuffer for the smaller images and writing them to the one ImageBuffer (likely keeping track of some grid and the bounds at which each smaller image would belong in the image), ensuring that the copy does not happen multiple times, as appending the images to one another repeatedly to create the full image is very inefficient.