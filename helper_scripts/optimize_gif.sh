#!/usr/bin/env bash

if [ "$#" -ne "4" ]; then
    echo "Usage: optimize_gif <input_gif_name> <fps> <gif_resolution> <output_gif_name>"
    echo "    <input_gif_name> is the name of the gif to optimize."
    echo "    <fps> is the output gif's fps."
    echo "    <gif_resolution> is the scale of the output gif."
    echo "    <output_gif_name> is the output gif (specify .gif at the end)"
    exit 1
fi

INPUT_GIF_NAME="$1"
FPS="$2"
GIF_RESOLUTION="$3"
OUTPUT_GIF_NAME="$4"

ffmpeg -y -i "$INPUT_GIF_NAME" -filter_complex "fps=$FPS,scale=$GIF_RESOLUTION:-1:flags=lanczos" "$OUTPUT_GIF_NAME"
