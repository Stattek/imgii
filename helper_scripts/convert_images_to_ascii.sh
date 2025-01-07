#!/usr/bin/env bash

if [ "$#" -ne "1" ]; then
    echo "Usage: convert_images_to_ascii <final_image_index>"
    echo "Assumes that the images are named image<num>.png and outputs the files as image<num>.txt"
    exit 1
fi

# have the end be however many images you have
END="$1"

for i in $(seq 1 $END); do
    jp2a --color --width=120 "image$i.png" >"image$i.txt"
done
