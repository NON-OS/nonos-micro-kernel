#!/bin/sh
# Rasterises the shared icon set to 8-bit alpha masks.
# The shell's tinted blit reads only the alpha channel, so colour is dropped
# here and supplied at draw time by the caller's tint.
set -e
cd "$(dirname "$0")"
SIZE=192
for svg in *.svg; do
  name="${svg%.svg}"
  magick -background none "$svg" -resize ${SIZE}x${SIZE} -depth 8 \
    -alpha extract -colorspace gray "GRAY:$name.a8"
done
