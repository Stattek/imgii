#!/usr/bin/env bash

if [ "$#" -ne "3" ]; then
    echo "Usage: convert_gif_to_pngs <input_png_name_format> <fps> <output_gif_name>"
    echo "input_png_name_format should be something like 'out_image%d.png'"
    exit 1
fi

INPUT_PNG_NAME_FORMAT="$1"
FPS="$2"
OUTPUT_GIF_NAME="$3"

ffmpeg -framerate "$FPS" -i "$INPUT_PNG_NAME_FORMAT" "$OUTPUT_GIF_NAME"
