#!/usr/bin/env bash

if [ ! -f output.ppm ]; then
    echo "output.ppm not found"
    exit 1
fi
convert output.ppm output.png
feh --force-aliasing output.png
rm output.ppm output.png
