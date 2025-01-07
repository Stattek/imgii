# ascii_to_png
- This is a not well optimized program that renders ascii text with ANSI color escape sequences that contain only one letter per each escape sequence. I made it for fun.
- This program does not work with non-colored ascii text.
- To attain ascii with this coloring, some ascii programs like `jp2a` can create ascii text files that work with this program, as they create ANSI color escape sequences for one character at a time.
- This program is multithreaded to make it faster, but it is very expensive. Beware of 100% CPU utilization.