#!/usr/bin/env bash

if [ "$#" -ne "2" ]; then
    echo "Usage: convert_gif_to_pngs <input_gif_name> <fps>"
    echo "Always outputs the images as 'image%d.png'"
    exit 1
fi

INPUT_GIF_NAME="$1"
FPS="$2"

ffmpeg -i "$INPUT_GIF_NAME" -vf "fps=$FPS/1" "out_image%d.png"
