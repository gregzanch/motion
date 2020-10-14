# motion

Note: *motion is a work in progress*

`motion` is a program which creates an animation using a JSON schema. `motion` utilizes the following libraries (via a rust wrapper):
- `ciaro`
- `ffmpeg`
- `librsvg`

```sh
# ffmpeg args

-f rawvideo 
-vcodec rawvideo
-pix_fmt argb
-s 720x480
-r 30 
-i - 

-vf format=yuv420p
-vcodec libx264
-profile:v high
-preset:v medium
-crf 18
-movflags faststart
-y "out.mp4"

```