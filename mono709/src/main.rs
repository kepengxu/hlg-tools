/*
 * Copyright 2023 William Swartzendruber
 *
 * This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a
 * copy of the MPL was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 *
 * SPDX-License-Identifier: MPL-2.0
 */

use std::{
    fs::File,
    io::{stdout, BufWriter, Write},
};
use dsp::{
    tf::*,
    tm::*,
    pixel::*,
};
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
        .arg(Arg::with_name("size")
            .long("size")
            .short("s")
            .value_name("COUNT")
            .help("The size of each dimension of the 3D LUT")
            .takes_value(true)
            .required(false)
            .default_value("64")
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
        .after_help(format!("This utility uses Oklab to generate a 3D LUT that will monochrome \
            BT.709.\n\n\
            Copyright © 2023 William Swartzendruber\n\
            Licensed under the Mozilla Public License 2.0\n\
            <{}>", env!("CARGO_PKG_REPOSITORY")).as_str())
        .get_matches();
    let title = matches.value_of("title");
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
    let tone_mapper = Bt2408ToneMapper::new(0.0283, 0.021875, ToneMapMethod::MaxRgb);

    writeln!(output, "# Generated by Mono709 {}", env!("CARGO_PKG_VERSION")).unwrap();
    if title.is_some() {
        writeln!(output, "TITLE \"{}\"", title.unwrap()).unwrap();
    }
    writeln!(output, "LUT_3D_SIZE {}", size).unwrap();

    for b in 0..size {
        for g in 0..size {
            for r in 0..size {

                let rgb_1 = RgbPixel
                {
                    red: r as f64 / (size - 1) as f64,
                    green: g as f64 / (size - 1) as f64,
                    blue: b as f64 / (size - 1) as f64,
                };
                let rgb_2 = rgb_1.with_each_channel(|x| pq_eotf(x));
                let rgb_3 = tone_mapper.map(rgb_2);
                let rgb_4 = rgb_3 * 0.457142857;
                let rgb_5 = rgb_4.with_each_channel(|x| sdr_o_to_e(x * 100.0));

                writeln!(output, "{} {} {}",
                    rgb_5.red as f32,
                    rgb_5.green as f32,
                    rgb_5.blue as f32,
                ).unwrap();
            }
        }
    }
}
