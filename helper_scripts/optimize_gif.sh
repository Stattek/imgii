#!/usr/bin/env bash

if [ "$#" -ne "3" ]; then
    echo "Usage: optimize_gif <input_gif_name> <fps> <gif_resolution> <output_image_name>"
    echo "Assumes that the input images are named outimage<num>.png and outputs the gif as <output_image_name>.gif"
    exit 1
fi

INPUT_GIF_NAME="$1"
FPS="$2"
GIF_RESOLUTION="$3"
OUTPUT_IMAGE_NAME="$4"

ffmpeg -y -i final_output.gif -filter_complex "fps=$(FPS),scale=$(GIF_RESOLUTION):-1:flags=lanczos" "($OUTPUT_IMAGE_NAME).gif"
