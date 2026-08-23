#!/bin/sh
# Rasterises the shell's line-art icon set to straight-alpha RGBA8.
# Paths are the ones the desktop mockup ships; the shell tints by alpha, so
# only the coverage channel of the output matters.
set -e
cd "$(dirname "$0")"
SIZE=192
emit() {
  name=$1; vb=$2; body=$3
  printf '<svg xmlns="http://www.w3.org/2000/svg" width="%s" height="%s" viewBox="%s" fill="none">%s</svg>' "$SIZE" "$SIZE" "$vb" "$body" > /tmp/_ico.svg
  magick -background none /tmp/_ico.svg -depth 8 -define quantum:format=unsigned "RGBA:$name.rgba"
}
S='stroke="#22C3F0" stroke-width="1.5"'
R='stroke-linecap="round"'
J='stroke-linejoin="round"'

emit terminal '0 0 20 20' "<rect x=\"1.4\" y=\"3\" width=\"17.2\" height=\"14\" rx=\"2.4\" $S/><path d=\"M5 8l2.6 2.2L5 12.4M9.6 12.8h5\" $S $R $J/>"
emit files '0 0 20 20' "<path d=\"M2 5.6A2 2 0 0 1 4 3.6h3.6l2 2.7H16a2 2 0 0 1 2 2v6.1a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5.6Z\" $S $J/>"
emit editor '0 0 20 20' "<path d=\"M5 2.4h6.6l3.8 3.8v11.4H5V2.4Z\" $S $J/><path d=\"M11.6 2.4v3.8h3.8\" $S $J/>"
emit settings '0 0 20 20' "<circle cx=\"10\" cy=\"10\" r=\"3.1\" $S/><path d=\"M10 1.6v2.6M10 15.8v2.6M1.6 10h2.6M15.8 10h2.6M4 4l1.9 1.9M14.1 14.1 16 16M16 4l-1.9 1.9M5.9 14.1 4 16\" $S $R/>"
emit processes '0 0 20 20' "<path d=\"M1.6 10.4h3l2.2-5.6 3 11.2 2.4-7.4 1.6 1.8h4.6\" $S $R $J/>"
emit about '0 0 20 20' "<circle cx=\"10\" cy=\"10\" r=\"8\" $S/><path d=\"M10 9v5M10 6.1v1.1\" $S $R/>"
emit calc '0 0 20 20' "<rect x=\"3.4\" y=\"1.8\" width=\"13.2\" height=\"16.4\" rx=\"2.2\" $S/><path d=\"M6.4 5.6h7.2M6.8 10h1M9.5 10h1M12.2 10h1M6.8 13.6h1M9.5 13.6h1M12.2 13.6h1\" $S $R/>"
emit wallet '0 0 20 20' "<rect x=\"1.8\" y=\"4.4\" width=\"16.4\" height=\"11.2\" rx=\"2.2\" $S/><path d=\"M1.8 8h16.4\" $S/>"
emit browser '0 0 20 20' "<circle cx=\"10\" cy=\"10\" r=\"8\" $S/><ellipse cx=\"10\" cy=\"10\" rx=\"3.4\" ry=\"8\" $S/><path d=\"M2.3 7.6h15.4M2.3 12.4h15.4\" $S/>"
emit audio_player '0 0 20 20' "<path d=\"M16.4 3 7 5.2v8.9\" $S $R/><circle cx=\"4.6\" cy=\"14.6\" r=\"2.6\" $S/><circle cx=\"16.4\" cy=\"12.6\" r=\"2.6\" $S/><path d=\"M16.4 3v9.6\" $S/>"
emit video_player '0 0 20 20' "<rect x=\"1.8\" y=\"3.6\" width=\"16.4\" height=\"12.8\" rx=\"2.2\" $S/><path d=\"M8.4 7.2v5.6l4.4-2.8-4.4-2.8Z\" $S $J/>"
emit clock '0 0 20 20' "<circle cx=\"10\" cy=\"10\" r=\"8\" $S/><path d=\"M10 5.2V10l3.3 2\" $S $R $J/>"
emit image_viewer '0 0 20 20' "<rect x=\"1.8\" y=\"3.6\" width=\"16.4\" height=\"12.8\" rx=\"2.2\" $S/><circle cx=\"6.8\" cy=\"8\" r=\"1.5\" $S/><path d=\"M2.6 14.8 7.6 9.8l3 3 2.6-2.4 4.2 4\" $S $R $J/>"
emit snake '0 0 20 20' "<path d=\"M3.4 5h6a3 3 0 0 1 0 6h-3a3 3 0 0 0 0 6h6.3\" $S $R $J/><circle cx=\"16.4\" cy=\"17\" r=\"1.2\" $S/>"

emit fs_folder '-1 -2 36 36' "<path d=\"M2 7a3 3 0 0 1 3-3h8l3 4h11a3 3 0 0 1 3 3v13a3 3 0 0 1-3 3H5a3 3 0 0 1-3-3V7Z\" $S $J/>"
emit fs_file '-1 -2 36 36' "<path d=\"M8 2h11l7 7v19H8V2Z\" $S $J/><path d=\"M19 2v7h7\" $S $J/>"
