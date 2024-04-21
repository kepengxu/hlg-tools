```
pq2hlg 2.0.0
William Swartzendruber <wswartzendruber@gmail.com>
Generates a Cube LUT for Converting from PQ to HLG

USAGE:
    pq2hlg [FLAGS] [OPTIONS] <OUTPUT-FILE>

FLAGS:
    -h, --help       Prints help information
    -p, --preview    Generates a black and white SDR preview LUT instead of a HLG one
    -V, --version    Prints version information

OPTIONS:
    -e, --exposure <FACTOR>                    Scales the exposure of the input video by the specified factor using
                                               Oklab
    -m, --max-cll <NITS>                       MaxCLL value of the input. [default: 1000]
    -r, --ref-white <NITS>                     Brightness of the input video stream's reference white level
    -s, --size <COUNT>                         The size of each dimension of the 3D LUT [default: 64]
    -t, --title <STRING>                       Title of the LUT
        --tone-map-method <tone-map-method>    Tone mapping method to use. [default: maxrgb]  [values: rgb, maxrgb]

ARGS:
    <OUTPUT-FILE>    Output Cube LUT file; use - for STDOUT

This utility follows the BT.2408 method for generating a PQ-to-HLG conversion LUT. If either --exposure or --ref-white
are provided, the exposure will either be scaled by the provided factor, or scaled to bring the provided reference white
level to 203 nits, using Oklab in both cases. This will cause the --max-cll value to be internally adjusted as well. If
the internal MaxCLL value then exceeds 1,000 nits, BT.2408 tone mapping will be applied to compress the input to 1,000
nits using either the maxRGB or R'G'B' method. From there, the signal will be converted to HLG. The generated LUTs are
completely full range with 0.0 representing minimum brightness and 1.0 representing maximum brightness.

Optionally, a preview LUT can be generated to convert the input to black and white SDR. This can be used to compare the
converted output to available BT.709 frames once they are also converted to black and white. In this way, --exposure can
be adjusted until the two sets of screenshots match as closely as possible.

Copyright © 2023 William Swartzendruber
Licensed under the Mozilla Public License 2.0
<https://github.com/wswartzendruber/hlg-tools>
```

# Introduction

**pq2hlg** generates 3D LUTs that will convert video streams from Perceptual Quantizer (PQ) HDR into Hybrid Log-Gamma (HLG) HDR. This can be used, for example, to convert 4K UltraHD discs to HLG thereby allowing for more universal playback on devices including SDR displays. While it would be convenient for PQ sources to have consistent brightness characteristics, this is typically not the case. As such, a unique LUT typically needs to be generated for each one.

Before beginning, the following points should be understood:

* HDR10 is simply PQ with attached static metadata.
* BT.2020 refers to wide color gamut and is used by PQ, HLG, and sometimes SDR.
* BT.709 refers to legacy color gamut and is used by sRGB, HDTV, and the original iteration of Blu-ray.
* Most of the work here is based on [ITU-R BT.2408](https://www.itu.int/pub/R-REP-BT.2408).
* Decryption is not addressed.

# Rationale

## PQ Weaknesses

While it is true that PQ (and therefore HDR10) offer a rather large amount of dynamic range, the format has two main weaknesses:

- PQ video streams **are** mastered for specific viewing environments.
- PQ video streams **cannot** be reliably rendered on SDR displays, whether BT.2020 or BT.709, with good results.

Regarding the first point above, PQ video streams are typically mastered for a viewing environment of just 5 nits. Any increase in ambient lighting causes the displayed image to appear darker than originally intended. This inherently limits what environments a PQ video can
be viewed in. Another way to look at this is that the viewer is expected to accommodate the presentation.

Regarding the second point above, PQ is inherently difficult here because nothing about the signal data indicates how SDR downconversion should be handled. This is necessary because applying the PQ gamma curve without processing will produce a picture that is simply too dim.
Static HDR10 metadata cannot help us here, either, because what we really need for SDR downconversion is the reference white level, but that is not signaled and varies greatly between productions.

## HLG Strengths

In light of these two issues, the United Kingdom's BBC and Japan's NHK cooperated to create an entirely new HDR format. This resulted in HLG which retains much of PQ's dynamic range capability while also focusing on flexibility and backwards compatibility:

- HLG video streams **are not** mastered for specific viewing environments.
- HLG video streams **can** be transparently rendered on BT.2020 SDR displays with no extra processing.
- HLG video streams **can** be rendered on BT.709 SDR displays with simple gamut mapping.

Regarding the first point above, HLG video streams are mastered in relative brightness. This means that instead of mastering the signal for a fixed viewing environment, HLG streams contain more generic signal data that the display then alters according to its settings. Ergo, if the viewing environment changes, the display can simply be adjusted and the same video can then be viewed again with these new settings. This is how SDR does things.

Regarding the second point above, HLG is inherently easy here because its gamma curve was designed for it. That is, a HLG video signal can be naively displayed on a modern BT.2020 SDR device with very acceptable results. And if a legacy BT.709 SDR device knows how to map BT.2020
color, a HLG presentation can then be viewed on such a device with very little indication that it wasn't originally mastered for BT.709 SDR to begin with. This last situation is something that has been continually improving as software support for BT.2020 to BT.709 conversion becomes more widely supported.

# Example

Here's an example of HDR10-to-HLG conversion using entirely free and open source software (FOSS). Specifically, we'll be converting *Alita: Battle Angel* from HDR10 to HLG. First we'll generate a Cube LUT specific to this movie's brightness characteristics and then we'll pass the LUT into FFmpeg to perform the actual conversion.

For this example, we'll assume the following are available in the active `PATH`:

1. hlg-tools-2.x.x
2. FFmpeg

We'll also assume the presence of our example movie in the current directory:

1. `pq.mkv`

## Generate a LUT

There are two things we need to know in order to generate a Cube LUT:

1. The input's MaxCLL value
2. The exposure adjustment needed

MaxCLL is essentially the peak brightness of a HDR10 video, which can typically be extracted from metadata. Meanwhile, the exposure adjustment is a factor that will scale the input's brightness to match the consistent brightness expected by HLG; it is crucial that this be set correctly for HLG to look good when played back on SDR. Alternatively, this can be set by providing the input's reference white value, if that is known.

In the case of *Alita: Battle Angel*, the MaxCLL value is **162** nits and it's reference white value is **48** nits. These values can be provided to **pq2hlg** like so:

`pq2hlg -m 162 -r 48 pq2hlg.cube`

We could also generate a similar LUT by providing an exposure adjustment factor instead of reference white:

`pq2hlg -m 162 -e 1.62 pq2hlg.cube`

In both cases, the resulting LUT is named `pq2hlg.cube`. We will be passing this into FFmpeg in the next step.

For other movies, see the [Movies Database](./Movies-Database). As that list in inherently non-exhaustive, see how to [Determine Picture Properties](./Determine-Picture-Properties) for how to find these yourself.

## Convert with FFmpeg

With our generated LUT in hand, we can now use FFmpeg (for instance) to convert our video.

Assuming the movie is available in a file called `pq.mkv`, the following command will get things done:

`ffmpeg -i pq.mkv -vf crop=3840:1600,format=rgb48le,lut3d=pq2hlg.cube,format=yuv420p10le -color_primaries bt2020 -color_trc bt2020-10 -colorspace bt2020nc -c:v libx265 -b:v 38M -preset slower -tune grain -x265-params atc-sei=18:pic_struct=0 -c:a aac -b:a 640K hlg.mkv`

This is, of course, merely an example and many personal preferences can be modified.