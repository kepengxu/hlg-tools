/*
 * Copyright © 2021 William Swartzendruber
 * Licensed under the Open Software License version 3.0
 */

use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
};
use tf::{Pixel, PqHlgMapper};
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

fn main() {

    let matches = app_from_crate!()
        .arg(Arg::with_name("title")
            .long("title")
            .short("t")
            .value_name("STRING")
            .help("Title of the LUT")
            .takes_value(true)
            .required(false)
            .validator(|value| {
                if value.contains("\"") {
                    return Err("Must not contain a double quote mark".to_string())
                }
                if value.len() > 242 {
                    return Err("Must not have a length greater than 242 bytes".to_string())
                }
                Ok(())
            })
        )
        .arg(Arg::with_name("ref-white")
            .long("ref-white")
            .short("r")
            .value_name("NITS")
            .help("Brightness of the input video stream's reference white level")
            .takes_value(true)
            .required(false)
            .default_value("203")
            .validator(|value| {
                let ref_white = value.parse::<f64>();
                if ref_white.is_err() {
                    return Err("Must be a floating point value".to_string())
                }
                let ref_white_value = ref_white.unwrap();
                if !ref_white_value.is_normal() {
                    return Err("Must be a normal number".to_string())
                }
                if !ref_white_value.is_sign_positive() {
                    return Err("Must be a positive number".to_string())
                }
                Ok(())
            })
        )
        .arg(Arg::with_name("max-channel")
            .long("max-channel")
            .short("m")
            .value_name("RATIO")
            .help("Linear ratio of the input video stream's brightest color channel")
            .takes_value(true)
            .required(false)
            .default_value("1.0")
            .validator(|value| {
                let max_channel = value.parse::<f64>();
                if max_channel.is_err() {
                    return Err("Must be a floating point value".to_string())
                }
                let max_channel_value = max_channel.unwrap();
                if !max_channel_value.is_normal() {
                    return Err("Must be a normal number".to_string())
                }
                if !max_channel_value.is_sign_positive() {
                    return Err("Must be a positive number".to_string())
                }
                if max_channel_value > 1.0 {
                    return Err("Must not exceed 1.0.".to_string())
                }
                Ok(())
            })
        )
        .arg(Arg::with_name("size")
            .long("size")
            .short("s")
            .value_name("COUNT")
            .help("The size of each dimension of the 3D LUT")
            .takes_value(true)
            .required(false)
            .default_value("32")
            .validator(|value| {
                let size = value.parse::<usize>();
                if size.is_err() {
                    return Err("Must be an unsigned integer value".to_string())
                }
                let size_value = size.unwrap();
                if size_value < 2 || size_value > 256 {
                    return Err("Must be between 2 and 256".to_string())
                }
                Ok(())
            })
        )
        .arg(Arg::with_name("output")
            .index(1)
            .value_name("OUTPUT-FILE")
            .help("Output Cube LUT file; use - for STDOUT")
            .required(true)
        )
        .after_help(format!("This utility follows the BT.2390 method for generating a \
            PQ-to-HLG conversion LUT. If a --ref-white value is supplied, the input will first \
            be linearly scaled to bring that level to 203 nits. The --max-channel value will \
            then be internally adjusted by that factor to reflect the effect of the scaling. \
            If the internal max-channel value then exceeds 0.1, BT.2390 tone mapping will be \
            applied to each color channel independently, otherwise, no tone mapping will be \
            necessary.\n\n\
            Copyright © 2021 William Swartzendruber\n\
            Licensed under the Open Software License version 3.0\n\
            <{}>", env!("CARGO_PKG_REPOSITORY")).as_str())
        .get_matches();
    let title = matches.value_of("title");
    let ref_white = matches.value_of("ref-white").unwrap().parse::<f64>().unwrap();
    let max_channel = matches.value_of("max-channel").unwrap().parse::<f64>().unwrap();
    let pq_hlg_mapper = PqHlgMapper::new(ref_white, max_channel);
    let size = matches.value_of("size").unwrap().parse::<usize>().unwrap();
    let output_value = matches.value_of("output").unwrap();
    let (mut stdout_write, mut file_write);
    let mut output = BufWriter::<&mut dyn Write>::new(
        if output_value == "-" {
            stdout_write = stdout();
            &mut stdout_write
        } else {
            file_write = File::create(output_value)
                .expect("Could not open output file for writing.");
            &mut file_write
        }
    );

    if title.is_some() {
        writeln!(output, "TITLE \"{}\"", title.unwrap()).unwrap();
    }

    writeln!(output, "# Generated by PQ2HLG {}", crate_version!()).unwrap();
    writeln!(output, "# ref-white: {}", ref_white).unwrap();
    writeln!(output, "# max-channel: {}", max_channel).unwrap();
    writeln!(output, "LUT_3D_SIZE {}", size).unwrap();

    for b in 0..size {
        for g in 0..size {
            for r in 0..size {

                let pixel = pq_hlg_mapper.map(Pixel {
                    red: r as f64 / (size - 1) as f64,
                    green: g as f64 / (size - 1) as f64,
                    blue: b as f64 / (size - 1) as f64,
                });

                writeln!(output, "{} {} {}",
                    pixel.red.min(1.0) as f32,
                    pixel.green.min(1.0) as f32,
                    pixel.blue.min(1.0) as f32,
                ).unwrap();
            }
        }
    }
}