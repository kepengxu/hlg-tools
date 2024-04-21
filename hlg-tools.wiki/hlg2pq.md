```
hlg2pq 2.0.0
William Swartzendruber <wswartzendruber@gmail.com>
Generates a Cube LUT for Converting from HLG to PQ

USAGE:
    hlg2pq [OPTIONS] <OUTPUT-FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --max-cll <NITS>    MaxCLL value of the output. [default: 1000]
    -s, --size <COUNT>      The size of each dimension of the 3D LUT [default: 64]
    -t, --title <STRING>    Title of the LUT

ARGS:
    <OUTPUT-FILE>    Output Cube LUT file; use - for STDOUT

This utility follows the BT.2408 method for generating a HLG-to-PQ conversion LUT according to the output brightness.

Copyright © 2023 William Swartzendruber
Licensed under the Mozilla Public License 2.0
<https://github.com/wswartzendruber/hlg-tools>
```

# Introduction

**hlg2pg** generates Cube LUTs that will convert video streams from Hybrid Log-Gamma (HLG) HDR into Perceptual Quantizer (PQ) HDR. This can be used, for example, for viewing originally-HLG content on PQ-only devices while preserving HDR, as HLG on PQ-only devices will typically play back as SDR.

Converting from HLG to PQ is typically a rather static process. As such, a single LUT can probably be generated that will work on multiple input files. Note that HLG input should have reference white at 75% signal level (203 nits in a reference environment).

# Example

Here's an example of HLG-to-PQ conversion using entirely free and open source software (FOSS). First we'll generate a Cube LUT and then we'll pass the LUT into FFmpeg to perform the actual conversion.

For this example, we'll assume the following are available in the active `PATH`:

1. hlg-tools-2.x.x
2. FFmpeg

We'll also assume the presence of an example HLG video in the current directory:

1. `hlg.mkv`

## Generate a LUT

Unlike going from PQ to HLG, this process is fairly straightforward:

`hlg2pq hlg2pq.cube`

As stated above, this Cube LUT (**hlg2pq.cube**) can be used with any properly-graded HLG video.

## Convert with FFmpeg

With our generated LUT in hand, we can now use FFmpeg (for instance) to convert our video.

Assuming the video is available in a file called `pq.mkv`, the following command will get things done:

`ffmpeg -i hlg.mkv -vf format=rgb48le,lut3d=hlg2pq.cube,format=yuv420p10le -color_primaries bt2020 -color_trc smpte2084 -colorspace bt2020nc -c:v libx265 -b:v 38M -preset slower -tune grain -c:a aac -b:a 256K pq.mkv`

This is, of course, merely an example and many personal preferences can be modified.