#!/usr/bin/env bash

if [ "$#" -ne "2" ]; then
    echo "Usage: convert_gif_to_pngs <fps> <gif_resolution> <output_image_name>"
    echo "Assumes that the input images are named outimage<num>.png and outputs the gif as <output_image_name>.gif"
    exit 1
fi

FPS=$(wc -l $1)
GIF_RESOLUTION=$(wc -l $2)
OUTPUT_IMAGE_NAME=$(wc -l <$3)

ffmpeg -y -i final_output.gif -filter_complex "fps=$(FPS),scale=$(GIF_RESOLUTION):-1:flags=lanczos" "($OUTPUT_IMAGE_NAME).gif"
