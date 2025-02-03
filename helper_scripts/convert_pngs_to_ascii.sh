#!/usr/bin/env bash

if [ "$#" -ne "3" ]; then
    echo "Usage: convert_images_to_ascii <image_name_prefix> <final_image_index> <width>"
    echo "Assumes that the images are named image<num>.png and outputs the files as image<num>.txt"
    exit 1
fi

# have the end be however many images you have
IMAGE_NAME_PREFIX="$1"
END="$2"
WIDTH="$3"

for i in $(seq 1 $END); do
    jp2a --color "--width=$WIDTH" "$IMAGE_NAME_PREFIX$i.png" >"$IMAGE_NAME_PREFIX$i.txt"
done
