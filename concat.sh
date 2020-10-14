#!/usr/bin/env bash

# cat res/out/*.png | ffmpeg -f "rawvideo" -vcodec "rawvideo" -pix_fmt "argb" -s "720x480" -r "30" -i - -vf "format=yuv420p" -vcodec "libx264" -profile:v "high" -preset:v "medium" -crf "18" -movflags "faststart" -y "out.mp4"
cat res/out/*.png | ffmpeg -f "image2pipe" -s "720x480" -r "30" -i - -vf "format=yuv420p" -vcodec "libx264" -profile:v "high" -preset:v "medium" -crf "18" -movflags "faststart" -y "out.mp4"