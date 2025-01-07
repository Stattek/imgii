#!/usr/bin/env bash

if [ "$#" -ne "2" ]; then
    echo "Usage: convert_gif_to_pngs <num_images>"
    echo "Assumes that the images are named image<num>.png and outputs the files as image<num>.txt"
    exit 1
fi

# have the end be however many images you have
END=$(wc -l <$1)

for i in $(seq 1 $END); do
    jp2a --color --width=120 "image$i.png" >"image$i.txt"
done
