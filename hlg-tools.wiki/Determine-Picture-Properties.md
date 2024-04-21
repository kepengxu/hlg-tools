# Introduction

In order to generate a Cube LUT, **pq2hlg** needs two parameters that can vary across productions:

1. MaxCLL
2. Exposure adjustment

Here we'll walk through determining each by hand using 2008's *Iron Man* as an example. The basic idea is that while UltraHD discs have widely varying levels here, legacy discs are much more consistent. We will effectively be using the legacy disc's levels as a reference point for determining the UltraHD disc's levels.

# Prerequisites

This guide will assume the following files exist in the current directory:

1. `pq.mkv` - A dump of the *Iron Man* UltraHD Blu-ray disc.
2. `sdr.mkv` - A dump of the *Iron Man* legacy Blu-ray disc.

It will also assume the presence of the following in `PATH`:

1. The extracted contents of **hlg-tools** version 2.1.0 or higher.
2. The `mediainfo` command line utility.

The example commands below assume Linux. If on Windows, replace `.sh` at the end of each script's file name with `.ps1`.

# Determine MaxCLL

First we're going to determine the movie's MaxCLL value. For this walkthrough, enter the following command:

`mediainfo pq.mkv`

In the output, look for the following entry:

`Maximum Content Light Level : 826 cd/m2`

In this primary case, the MaxCLL value is **826**. If in a different production no **Maximum Content Light Level** entry is present, then look for the following entry instead:

`Mastering display luminance : min: 0.0050 cd/m2, max: 4000 cd/m2`

In this secondary case, use the larger value, which is **4000** in the example above.

Finally, if neither of these are present, simply choose **1000** as a last resort.

# Determine Exposure Adjustment

Now we need to determine the exposure adjustment for this specific UltraHD disc. Unfortunately, this involves some trial and error. But if done properly, the end result is typically quite pleasing. This is where `sdrprev.sh` and `hlgprev.sh` come into play.

The purpose of the `sdrprev.sh` script is to export a single black-and-white frame from the legacy disc. This isn't a naive sRGB-based monochroming of the image, but one done using the Oklab color model.

Before we do that, though, we need to open the `sdr.mkv` dump and sift through it until we find a few ideal frames. These frames are characterized by having some non-reflective object or surface that is well-lit with diffused lighting. In other words, not something directly exposed to sunlight, but not something in the shadows. Medium and light skintones work well here. A good indicator is if the object is curved or has a complex shape, yet also has very little variance in shading across its surface. Avoid objects in the dark even if they themselves are well-lit.

**NOTE:** Sometimes an UltraHD disc and its legacy counterpart will have offset timestamps. In the case of *Iron Man*, the legacy disc (`sdr.mkv`) is exactly two seconds ahead of the UltraHD disc (`pq.mkv`).

For *Iron Man*, I have chosen three specific objects at three timestamps:

* Pepper's face at **44:46**
* Rhodes' face at **46:04**
* Phil's face at **1:53:57**

They are shown below:

![Pepper's face at 44:46](https://github.com/wswartzendruber/hlg-tools/assets/1069654/04244fd2-1615-48da-a92d-c833c67ad0bc)

![Rhodes' face at 46:04](https://github.com/wswartzendruber/hlg-tools/assets/1069654/a4b39c25-d002-49f5-8c73-410e7f7ed574)

![Phil's face at 1:53:57](https://github.com/wswartzendruber/hlg-tools/assets/1069654/1bd34a95-bb92-43f7-9ad3-adc517f054fc)

To start, let's export the frames from the legacy disc by entering these commands:

1. `sdrprev.sh sdr.mkv 44:46 sdr-1.png`
2. `sdrprev.sh sdr.mkv 46:04 sdr-2.png`
3. `sdrprev.sh sdr.mkv 1:53:57 sdr-3.png`

And again, the frames are shown below:

![Pepper's face at 44:46 (SDR preview)](https://github.com/wswartzendruber/hlg-tools/assets/1069654/7e008414-0b6c-473b-bd02-c55e76c33f1c)

![Rhodes' face at 46:04 (SDR preview)](https://github.com/wswartzendruber/hlg-tools/assets/1069654/0f6ec461-c12a-42d6-9440-f645dac78f85)

![Phil's face at 1:53:57 (SDR preview)](https://github.com/wswartzendruber/hlg-tools/assets/1069654/2b84b4c2-6b89-4356-be35-91f5cbd8b6c4)

Now that we have these frames, our goal is to use the `hlgprev.sh` script to get the objects we've chosen from the legacy disc to match the corresponding objects on the UltraHD disc as closely as possible. This is the part that involves trial and error and it is typically not an exact science.

Start with the first frame by entering the following command:

`hlgprev.sh pq.mkv 826 1.0 44:44 hlg-1-1.0.png`

Note the following:

1. The `826` parameter refers to the MaxCLL value we extracted earlier.
2. The `1.0` value is the initial exposure adjustment; it might not be correct.
3. Frames on this particular UltraHD disc are two seconds behind its legacy counterpart.

Below is a composite frame of both the SDR preview and the initial (exposure 1.0) HLG preview. The frame is split vertically down the middle of Pepper's face. HLG (`hlg-1-1.0.png`) is on the left and SDR (`sdr-1.png`) is on the right. In this initial case, it is virtually impossible to tell that this isn't just a single frame. **This is a very big indicator that the exposure adjustment we've tried works really well here.**

![Pepper's face at 44:44/46 (HLG/SDR composite, exposure 1.0)](https://github.com/wswartzendruber/hlg-tools/assets/1069654/62c29d54-5435-4818-ad19-f583d2615634)

The first frame is taken care of and we know that `1.0` is a good exposure value for it. Now we move onto the second frame and try the same value there:

`hlgprev.sh pq.mkv 826 1.0 46:02 hlg-2-1.0.png`

And a similarly composited result of the initial HLG and SDR frames are shown below. Here, however, we can see a very slight difference between the two halves of Rhodes' face.

![Rhodes' face at 46:02/04 (HLG/SDR composite, exposure 1.0)](https://github.com/wswartzendruber/hlg-tools/assets/1069654/b6cd0b9e-a48b-49e3-bbec-5cdd031f34c4)

It's not a big difference, though, so a small adjustment should suffice. Let's try again with an exposure value of 1.04:

`hlgprev.sh pq.mkv 826 1.04 46:02 hlg-2-1.04.png`

And this second attempt yields an essentially perfect match:

![Rhodes' face at 46:02/04 (HLG/SDR composite, exposure 1.04)](https://github.com/wswartzendruber/hlg-tools/assets/1069654/514f256c-030f-49ab-abca-85960c1bb854)

And so an exposure value of `1.04` works quite well here. Let's move onto the final frame:

`hlgprev.sh pq.mkv 826 1.0 1:53:55 hlg-3-1.0.png`

And again we see that an exposure of 1.0 is going to be too dark here. The split is just to the left of Phil's eye:

![Phil's face at 1:53:55/57 (HLG/SDR composite, exposure 1.0)](https://github.com/wswartzendruber/hlg-tools/assets/1069654/f0e6753c-ba38-499c-b5a4-c446f7beface)

An exposure of `1.16` ends up being pretty close:

`hlgprev.sh pq.mkv 826 1.16 1:53:55 hlg-3-1.16.png`

It yields the following comparison:

![Phil's face at 1:53:55/57 (HLG/SDR composite, exposure 1.16)](https://github.com/wswartzendruber/hlg-tools/assets/1069654/51b0419c-5c2d-4e60-9b3c-ef1a9d1c120d)

With these three frames now matched fairly well, we are left with three total exposure values:

1. `1.0`
2. `1.04`
3. `1.16`

The final exposure adjustment value takes the average of the three:

`(1.0 + 1.04 + 1.16) ÷ 3 ≈ 1.067`

Note that four significant digits is quite sufficient.

# Result

This process leaves us with `826` for MaxCLL and `1.067` for exposure adjustment. We can invoke **pq2hlg** with these parameters like so:

`pq2hlg -m 826 -e 1.067 pq2hlg.cube`
